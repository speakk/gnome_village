extends TileMap

class_name MainMap

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")
#@onready var HOVER_RECT := preload("res://src/map/hover_rect.tscn")

const MAP_SIZE_X: int = 80
const MAP_SIZE_Y: int = 40
const CELL_SIZE := Vector2i(24, 24)

#var construction_item_id: Variant # Items.Id | null

@onready var ui_action_handlers: Dictionary = {
	UiAction.UiActionId.Build: preload("res://src/map/ui_action_handlers/build_handler.gd").new(),
	UiAction.UiActionId.Dismantle: preload("res://src/map/ui_action_handlers/dismantle_handler.gd").new()
}

enum MapActions {
	Build, Dismantle, None
}

#var current_action: Globals.PlayerAction = Globals.PlayerAction.None
#var current_action_params: Dictionary

var selected_ui_action: UiAction

enum Layers {
	Ground, Building, Blueprint
}

var map_entities := {} as Dictionary

func prepare_for_load() -> void:
	map_entities.clear()
	clear_layer(Layers.Building)
	clear_layer(Layers.Blueprint)
	#current_action = MapActions.None
	#construction_item_id = null

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
	
	ui_action_handlers[UiAction.UiActionId.Build].build_issued.connect(_place_blueprint)
	ui_action_handlers[UiAction.UiActionId.Dismantle].dismantle_issued.connect(_dismantle_in_position)

	var world_center := Vector2(MAP_SIZE_X * CELL_SIZE.x / 2, MAP_SIZE_Y * CELL_SIZE.y / 2)

	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			#set_cell(Layers.Ground, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
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

func _dismantle_in_position(tile_position: Vector2i) -> void:
	var entities := get_map_entities(tile_position)
	for entity in entities as Array[Node]:
		entity as ItemOnGround
		if entity.item.can_be_dismantled and not entity.reserved_for_dismantling:
			Events.dismantle_issued.emit(entity)

func _handle_map_action(tile_position: Vector2i) -> void:
	if not selected_ui_action:
		return
	
	if ui_action_handlers.has(selected_ui_action.ui_action_id):
		var handler := ui_action_handlers[selected_ui_action.ui_action_id] as UiActionHandler
		handler.handle_action(selected_ui_action, tile_position, $SelectionDraw, is_mouse_pressed, is_mouse_2_pressed)

func _process(delta: float) -> void:
	var tile_position: Vector2i = local_to_map(get_local_mouse_position())
	
	if is_mouse_2_pressed:
		_cancel_blueprint(tile_position)
	
	_handle_map_action(tile_position)

func _place_blueprint(tile_position: Vector2i, item_id: Items.Id) -> void:
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


var is_mouse_pressed := false
var is_mouse_2_pressed := false

# TODO: You could use an Area2D and the input_event in that to handle this instead
func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_index == 1:
				is_mouse_pressed = true
			else:
				is_mouse_2_pressed = true
		else:
			is_mouse_pressed = false
			is_mouse_2_pressed = false
		

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
