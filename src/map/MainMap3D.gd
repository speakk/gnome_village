class_name MainMap3D extends Node3D

@export var clear_on_load: bool = false

@onready var grid: GridMap = $GridMap
@onready var blueprint_grid: GridMap = $BlueprintGridMap
@onready var ground_grid: GridMap = $GroundGrid

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

const MAP_SIZE_X: int = 200
const MAP_SIZE_Y: int = 150
const CELL_SIZE := Vector2(1, 1)

@onready var map_tile_selector := $MapTileSelector as MapTileSelector

var selected_ui_action: UiAction

var action_handlers: Dictionary = {
	UiAction.UiActionId.Build: _place_blueprint,
	UiAction.UiActionId.Dismantle: _dismantle_in_position,
	UiAction.UiActionId.ZoneAddTiles: _zone_add_tiles
}

var secondary_action_handlers: Dictionary = {
	UiAction.UiActionId.Build: _cancel_blueprint
}

enum Layers {
	Ground, Building, Blueprint
}

var map_entities := {} as Dictionary

func prepare_blueprint_grid() -> void:
	var original_mesh_library := grid.mesh_library
	var mesh_ids := original_mesh_library.get_item_list()
	
	var blueprint_mesh_library := MeshLibrary.new()
	
	for mesh_id in mesh_ids:
		var original_mesh := original_mesh_library.get_item_mesh(mesh_id)
		var new_mesh := original_mesh.duplicate(true) as Mesh
		var surface_count := new_mesh.get_surface_count()
		for surface_id in surface_count:
			var material := new_mesh.surface_get_material(surface_id) as BaseMaterial3D
			material.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
			material.albedo_color = Color(0.6,0.6,1, 0.5)
		
		var item_id := blueprint_mesh_library.get_last_unused_item_id()
		blueprint_mesh_library.create_item(item_id)
		blueprint_mesh_library.set_item_mesh(item_id, new_mesh)
	
	blueprint_grid.mesh_library = blueprint_mesh_library

func prepare_for_load() -> void:
	if clear_on_load:
		map_entities.clear()
		grid.clear()
		blueprint_grid.clear()
		
		
		var world_center := Vector2(MAP_SIZE_X * CELL_SIZE.x / 2, MAP_SIZE_Y * CELL_SIZE.y / 2)
		var water_area_y_size := 40
		var shore_wave_frequency := 0.05
		var shore_wave_depth := 6
		
		for x in MAP_SIZE_X:
			for y in MAP_SIZE_Y:
				if y < MAP_SIZE_Y - water_area_y_size - sin(x * shore_wave_frequency) * shore_wave_depth:
					ground_grid.set_cell_item(Vector3i(x - MAP_SIZE_X/2, 0, y - MAP_SIZE_Y/2), 0)
				else:
					ground_grid.set_cell_item(Vector3i(x - MAP_SIZE_X/2, 0, y - MAP_SIZE_Y/2), 1)

func add_map_entity(coordinate: Vector2i, item_on_ground: Node3D) -> void:
	if not map_entities.has(coordinate):
		map_entities[coordinate] = []
	
	if not map_entities[coordinate].has(item_on_ground):
		map_entities[coordinate].append(item_on_ground)

func remove_map_entity(coordinate: Vector2i, item_on_ground: Node3D) -> void:
	if map_entities.has(coordinate):
		map_entities[coordinate].erase(item_on_ground)

func get_map_entities(coordinate: Vector2i, items_only: bool = false) -> Array[Node3D]:
	var result: Array[Node3D]
	if map_entities.has(coordinate):
		result.assign(map_entities[coordinate])
		result = result.filter(func(entity: Node3D) -> bool:
			if items_only:
				return entity is ItemOnGround
			return true
			)
	return result

func is_coordinate_occupied(coordinate: Vector2i) -> bool:
	if not map_entities.has(coordinate):
		return false
	
	var entities := map_entities[coordinate] as Array
	
	for item_on_ground in entities as Array[ItemOnGround]:
		if item_on_ground is ItemOnGround:
			var container := item_on_ground.component_container
			if container.has_component(Components.Id.Solid) or container.has_component(Components.Id.Constructable):
				return true
	
	return false
	

func _ready() -> void:
	prepare_for_load()
	
	map_tile_selector.tiles_selected.connect(_tiles_selected)
	map_tile_selector.tiles_selected_secondary.connect(_tiles_selected_secondary)

	Events.terrain_placed.connect(_terrain_placed)
	Events.terrain_cleared.connect(_terrain_cleared)
	
	Events.world_position_changed.connect(func(entity: Node3D, old_position: Vector3, new_position: Vector3) -> void:
			var old_coordinate := global_position_to_coordinate(old_position)
			remove_map_entity(old_coordinate, entity)
			
			var coordinate := global_position_to_coordinate(new_position)
			add_map_entity(coordinate, entity)
			)
		
	Events.item_removed_from_ground.connect(func(item: ItemOnGround) -> void:
			var coordinate := global_position_to_coordinate(item.global_position)
			remove_map_entity(coordinate, item)
	)
	
	Events.ui_action_selected.connect(_handle_ui_action_selection)
	
	prepare_blueprint_grid()
	
	Events.map_ready.emit(self)
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			var final_x: int = x - MAP_SIZE_X/2
			var final_y: int = y - MAP_SIZE_Y/2
			if ground_grid.get_cell_item(Vector3i(final_x, 0, final_y)) != 0:
				PathFinder.set_coordinate_invalid(Vector2i(final_x, final_y))

func _handle_ui_action_selection(new_ui_action: UiAction) -> void:
	selected_ui_action = new_ui_action

func _tiles_selected(coordinates: Array[Vector2i]) -> void:
	if selected_ui_action:
		action_handlers[selected_ui_action.ui_action_id].call(coordinates)
	else:
		select_next_entity(coordinates)

func clear_selections() -> void:
	for entities: Array in map_entities.values():
		for entity: Variant in entities:
			if entity.component_container.has_component(Components.Id.Selectable):
				entity.component_container.get_by_id(Components.Id.Selectable).selected = false

func select_next_entity(coordinates: Array[Vector2i]) -> void:
	if coordinates.size() == 1:
		print("Select next entity")
		var entity_to_select: Node3D
		var coordinate := coordinates[0]
		var entities := get_map_entities(coordinate)
		for entity in entities:
			print("Going though entity", entity)
			if entity.component_container.has_component(Components.Id.Selectable):
				print("Had selectable")
				if not entity.component_container.get_by_id(Components.Id.Selectable).selected:
					entity_to_select = entity
					break
		
		
		if not entity_to_select:
			if entities.size() > 1:
				entity_to_select = entities[0]
			elif entities.size() == 1:
				return
		
		clear_selections()
		
		if entity_to_select:
			print("Setting as selected")
			entity_to_select.component_container.get_by_id(Components.Id.Selectable).selected = true
	
	else:
		clear_selections()
		for coordinate in coordinates:
			var entities := get_map_entities(coordinate)
			for entity in entities:
				if entity.component_container.has_component(Components.Id.Selectable):
					entity.component_container.get_by_id(Components.Id.Selectable).selected = true

func _tiles_selected_secondary(coordinates: Array[Vector2i]) -> void:
	print("Secondary called", coordinates)
	if selected_ui_action:
		if secondary_action_handlers.has(selected_ui_action.ui_action_id):
			secondary_action_handlers[selected_ui_action.ui_action_id].call(coordinates)

func _dismantle_in_position(coordinates: Array[Vector2i]) -> void:
	for tile_position in coordinates:
		var entities := get_map_entities(tile_position)
		for entity in entities as Array[Node]:
			if entity is ItemOnGround:
				if entity.item.can_be_dismantled and not entity.reserved_for_dismantling:
					Events.dismantle_issued.emit(entity)

func _zone_add_tiles(coordinates: Array[Vector2i]) -> void:
	var zone := (selected_ui_action as UiAction.ZoneAddTiles).zone
	zone.add_coordinates(coordinates)

func _create_placement_juice(item_on_ground: ItemOnGround, index: int) -> void:
	var container: ComponentContainer = item_on_ground.component_container
	var node: Node3D
	var removed_component: Component
	var position_correction := Vector3(0, 0, 0)
	if container.has_component(Components.Id.Terrain):
		var terrain_component: TerrainComponent = container.get_by_id(Components.Id.Terrain)
		container.remove_component(Components.Id.Terrain)
		node = MeshInstance3D.new()
		node.mesh = blueprint_grid.mesh_library.get_item_mesh(terrain_component.mesh_id)
		removed_component = terrain_component
		position_correction = Vector3(0, 0.5, 0)
	elif container.has_component(Components.Id.Scene):
		var scene_component: SceneComponent = container.get_by_id(Components.Id.Scene)
		container.remove_component(Components.Id.Scene)
		node = scene_component.scene.instantiate()
		removed_component = scene_component
	
	if not node:
		return
	
	await get_tree().create_timer(float(index) * 0.03).timeout
	add_child(node)
	var pos: Vector3 = container.get_by_id(Components.Id.WorldPosition).current_position + position_correction
	node.global_position = pos + Vector3(0, 1.0, 0)
	$PlacementJuicePlayer.pitch_scale = 1 + randf_range(-0.1, 0.1)
	$PlacementJuicePlayer.play()
	await create_tween().tween_property(node, "global_position", pos, 0.5).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_BOUNCE).finished
	node.queue_free()
	container.add_component(removed_component)

func _place_blueprint(coordinates: Array[Vector2i]) -> void:
	var item_id := (selected_ui_action as UiAction.Build).item_id
	var _index: int = 0
	for tile_position in coordinates:
		if not is_coordinate_occupied(tile_position):
			var blueprint := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			Events.request_entity_add.emit(blueprint)
			blueprint.initialize(item_id)
			WorldPositionComponent.set_world_position(blueprint, coordinate_to_global_position(tile_position))
			blueprint.component_container.add_component(BlueprintComponent.new())
			Events.blueprint_placed.emit(tile_position, blueprint)
			_create_placement_juice(blueprint, _index)
			_index += 1

func _cancel_blueprint(coordinates: Array[Vector2i]) -> void:
	for coordinate in coordinates:
		var entities := get_map_entities(coordinate)
		for entity in entities:
			if entity.component_container.has_component(Components.Id.Blueprint):
				entity.queue_free()

func _terrain_placed(coordinate: Vector2i, mesh_id: MapMeshes.Id, blueprint: bool) -> void:
	var grid_map: GridMap = grid if not blueprint else blueprint_grid
	grid_map.set_cell_item(Globals.extend_vec2i(coordinate), 0)

func _terrain_cleared(coordinate: Vector2i, blueprint: bool) -> void:
	var grid_map: GridMap = grid if not blueprint else blueprint_grid
	grid_map.set_cell_item(Globals.extend_vec2i(coordinate), -1)

# TODO: Unholy
func get_random_coordinate(accept_occupied: bool = true) -> Vector2i:
	while(true):
		var random_position := Vector2i(randi_range(1, MAP_SIZE_X-1), randi_range(1, MAP_SIZE_Y-1)) - Vector2i(MAP_SIZE_X, MAP_SIZE_Y) / 2
		if not PathFinder.is_position_solid(random_position):
			return random_position
	
	return Vector2i(0, 0)

func coordinate_to_global_position(coordinate: Vector2i) -> Vector3:
	var original_grid_pos := grid.to_global(grid.map_to_local(Vector3(coordinate.x, 0, coordinate.y)))
	return Vector3(original_grid_pos.x, 0, original_grid_pos.z)

func global_position_to_coordinate(_global_position: Vector3) -> Vector2i:
	var coordinate: Vector3i = grid.local_to_map(grid.to_local(_global_position))
	return Vector2i(coordinate.x, coordinate.z)
