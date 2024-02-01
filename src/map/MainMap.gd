extends TileMap

class_name MainMap

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

const MAP_SIZE_X: int = 80
const MAP_SIZE_Y: int = 40
const CELL_SIZE := Vector2i(24, 24)

@onready var map_tile_selector := $MapTileSelector as MapTileSelector

var selected_ui_action: UiAction

var action_handlers: Dictionary = {
	UiAction.UiActionId.Build: _place_blueprint,
	UiAction.UiActionId.Dismantle: _dismantle_in_position,
	UiAction.UiActionId.ZoneAddTiles: _zone_add_tiles
}

enum Layers {
	Ground, Building, Blueprint
}

var map_entities := {} as Dictionary

func prepare_for_load() -> void:
	map_entities.clear()
	clear_layer(Layers.Building)
	clear_layer(Layers.Blueprint)

func add_map_entity(coordinate: Vector2i, item_on_ground: ItemOnGround) -> void:
	if not map_entities.has(coordinate):
		map_entities[coordinate] = []
	
	if not map_entities[coordinate].has(item_on_ground):
		map_entities[coordinate].append(item_on_ground)

func remove_map_entity(coordinate: Vector2i, item_on_ground: ItemOnGround) -> void:
	if map_entities.has(coordinate):
		map_entities[coordinate].erase(item_on_ground)

func get_map_entities(coordinate: Vector2i) -> Array[ItemOnGround]:
	var result: Array[ItemOnGround]
	if not map_entities.has(coordinate):
		return result
		
	result.assign(map_entities[coordinate])
	return result

func is_coordinate_occupied(coordinate: Vector2i) -> bool:
	if not map_entities.has(coordinate):
		return false
	
	var entities := map_entities[coordinate] as Array
	
	for item_on_ground in entities as Array[ItemOnGround]:
		if item_on_ground.item.can_be_constructed:
			return true
	
	return false
	

func _ready() -> void:
	add_layer(Layers.Ground)
	add_layer(Layers.Building)
	add_layer(Layers.Blueprint)
	
	set_layer_z_index(Layers.Building, 1)
	
	map_tile_selector.tiles_selected.connect(_tiles_selected)

	var world_center := Vector2(MAP_SIZE_X * CELL_SIZE.x / 2, MAP_SIZE_Y * CELL_SIZE.y / 2)

	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			if world_center.distance_to(Vector2(x * CELL_SIZE.x, y * CELL_SIZE.y)) < 400:
				set_cells_terrain_connect(Layers.Ground, [Vector2i(x, y)], 1, 0)
	
	set_layer_modulate(Layers.Blueprint, Color(0.5, 0.5, 1.0, 0.5))
	
	Events.terrain_placed.connect(_terrain_placed)
	Events.terrain_cleared.connect(_terrain_cleared)
	
	Events.item_placed_on_ground.connect(func(item: ItemOnGround, item_position: Vector2) -> void:
			var coordinate := global_position_to_coordinate(item_position)
			add_map_entity(coordinate, item)
	)
	
	Events.item_removed_from_ground.connect(func(item: ItemOnGround) -> void:
			var coordinate := global_position_to_coordinate(item.global_position)
			remove_map_entity(coordinate, item)
	)
	
	Events.ui_action_selected.connect(_handle_ui_action_selection)
	
	Events.map_ready.emit(self)
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			if get_cell_source_id(Layers.Ground, Vector2i(x, y)) < 0:
				PathFinder.set_coordinate_invalid(Vector2i(x, y))

func _handle_ui_action_selection(new_ui_action: UiAction) -> void:
	selected_ui_action = new_ui_action

func _tiles_selected(coordinates: Array[Vector2i]) -> void:
	if selected_ui_action:
		action_handlers[selected_ui_action.ui_action_id].call(coordinates)

func _dismantle_in_position(coordinates: Array[Vector2i]) -> void:
	for tile_position in coordinates:
		var entities := get_map_entities(tile_position)
		for entity in entities as Array[Node]:
			entity as ItemOnGround
			if entity.item.can_be_dismantled and not entity.reserved_for_dismantling:
				Events.dismantle_issued.emit(entity)

func _zone_add_tiles(coordinates: Array[Vector2i]) -> void:
	var zone := (selected_ui_action as UiAction.ZoneAddTiles).zone
	zone.add_coordinates(coordinates)

func _process(delta: float) -> void:
	var tile_position: Vector2i = local_to_map(get_local_mouse_position())
	#
	#if is_mouse_2_pressed:
		#_cancel_blueprint(tile_position)

func _place_blueprint(coordinates: Array[Vector2i]) -> void:
	var item_id := (selected_ui_action as UiAction.Build).item_id
	for tile_position in coordinates:
		if not is_coordinate_occupied(tile_position):
			var blueprint := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			blueprint.global_position = coordinate_to_global_position(tile_position)
			get_tree().root.get_node("Main").get_node("Entities").add_child(blueprint)
			blueprint.initialize(item_id, 1, ItemOnGround.ItemState.Blueprint)
			Events.blueprint_placed.emit(tile_position, blueprint)

func _cancel_blueprint(tile_position: Vector2i) -> void:
	var entities := get_map_entities(tile_position)
	for entity in entities:
		if entity.current_state == ItemOnGround.ItemState.Blueprint:
			Events.blueprint_cancel_issued.emit(entity)
			print("Removing at: ", tile_position)

func coordinate_to_global_position(coordinate: Vector2i) -> Vector2:
	return to_global(map_to_local(coordinate))

func global_position_to_coordinate(_global_position: Vector2) -> Vector2i:
	return local_to_map(to_local(_global_position))

func _terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool, item_on_ground: ItemOnGround) -> void:
	set_cells_terrain_connect(target_layer, [coordinate], terrain_set_id, terrain_id)

func _terrain_cleared(coordinate: Vector2i, target_layer: MainMap.Layers, tileset_source_id: int) -> void:
	set_cells_terrain_connect(target_layer, [coordinate], tileset_source_id, -1)

func is_vacant_coordinate(coordinate: Vector2i) -> bool:
	var has_ground := get_cell_source_id(Layers.Ground, coordinate) >= 0
	return not PathFinder.is_position_solid(coordinate) and has_ground

# TODO: Unholy
func get_random_coordinate(accept_occupied: bool = true) -> Vector2i:
	while(true):
		var random_position := Vector2i(randi_range(1, MAP_SIZE_X-1), randi_range(1, MAP_SIZE_Y-1))
		if not PathFinder.is_position_solid(random_position):
			return random_position
	
	return Vector2i(0, 0)
