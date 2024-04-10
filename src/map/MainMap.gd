class_name MainMap extends Node3D

@export var clear_on_load: bool = false
@export var rock_placement_noise: Noise
@export var grass_placement_noise: Noise
@export var grass_color0: Color
@export var grass_color1: Color
@export var grass_color2: Color
@export var grass_color3: Color
@export var grass_prob_curve: Curve

@onready var grid: GridMap = $GridMap
@onready var blueprint_grid: GridMap = $BlueprintGridMap
@onready var ground_grid: GridMap = $GroundGrid
@onready var grass_multi_mesh: MultiMeshInstance3D = $GrassMultiMesh

@onready var grid_definitions := {
	grid = { map = grid, items = AboveGroundCells.values()},
	blueprint_grid = { map = blueprint_grid, items = AboveGroundCells.values()},
	ground_grid = { map = ground_grid, items = GroundCells.values()},
}


const MAP_SIZE_X: int = 200
const MAP_SIZE_Y: int = 150
const CELL_SIZE := Vector2(1, 1)

enum GroundCells {
	Dirt = 0,
	Water = 1,
	Grass1 = 2,
	Grass2 = 3,
	Grass3 = 4,
	RiverWater = 5,
	RiverBank = 6
}

enum AboveGroundCells {
	WoodenWall = 0,
	Rock = 1,
	Rock2 = 2
}

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

func _create_grass() -> void:
	var multi_mesh := MultiMesh.new()
	multi_mesh.transform_format = MultiMesh.TRANSFORM_3D
	multi_mesh.mesh = preload("res://assets/blender_models/foliage/grass_blade_mesh.res")
	multi_mesh.use_colors = true
	multi_mesh.mesh.surface_get_material(0).vertex_color_use_as_albedo = true
	var mat := multi_mesh.mesh.surface_get_material(0) as StandardMaterial3D
	mat.cull_mode = BaseMaterial3D.CULL_DISABLED
	mat.diffuse_mode = BaseMaterial3D.DIFFUSE_TOON
	var amount_of_tries: int = 800000
	
	var transforms: Array[Transform3D]
	var colors: PackedColorArray
	
	for i in amount_of_tries:
		var random_position := Vector3(randf_range(-MAP_SIZE_X/2, MAP_SIZE_X/2), 0, randf_range(-MAP_SIZE_Y/2, MAP_SIZE_Y/2))
		var noise_value := grass_placement_noise.get_noise_2d(
			random_position.x,
			random_position.z)
		
		var grid_pos := Globals.extend_vec2i(global_position_to_coordinate(random_position))
		var norm_value := remap(noise_value, -1, 1, 0, 1)
		var base_color: Color
		
		var cell_item := ground_grid.get_cell_item(grid_pos)
		if cell_item != GroundCells.Water and cell_item != GroundCells.RiverWater and cell_item != GroundCells.RiverBank:
			base_color = grass_color0
			if noise_value > 0 and noise_value < 0.2:
				ground_grid.set_cell_item(grid_pos, 4)
				base_color = grass_color1
			elif noise_value > 0.2 and noise_value < 0.4:
				ground_grid.set_cell_item(grid_pos, 2)
				base_color = grass_color2
			elif noise_value > 0.4:
				ground_grid.set_cell_item(grid_pos, 3)
				base_color = grass_color3
				
			
			if randf() > grass_prob_curve.sample(norm_value):
				continue
			
			var blade_transform := Transform3D(Basis(), random_position)
			blade_transform = blade_transform.rotated_local(Vector3.FORWARD, randf_range(-PI/3, PI/3))
			var scale_no := randf_range(0.4, 0.7)
			blade_transform = blade_transform.scaled_local(Vector3(scale_no, scale_no, scale_no))
			transforms.append(blade_transform)
			var color := base_color
			color.v = color.v - norm_value * 0.2 - 0.1
			colors.append(color)
				
	
	multi_mesh.instance_count = transforms.size()
	print("GRASS SIZE", multi_mesh.instance_count)
	
	for i in multi_mesh.instance_count:
		multi_mesh.set_instance_transform(i, transforms[i])
		multi_mesh.set_instance_color(i, colors[i])
	
	grass_multi_mesh.multimesh = multi_mesh
	
	#multi_mesh.instance_count = amount
	
	
	#multi_mesh.multimesh 

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

func clear_everything() -> void:
	map_entities.clear()
	grid.clear()
	blueprint_grid.clear()
	ground_grid.clear()
	PathFinder._reset()

func create_world() -> void:
	clear_everything()
	
	var world_center := Vector2(MAP_SIZE_X * CELL_SIZE.x / 2, MAP_SIZE_Y * CELL_SIZE.y / 2)
	var water_area_y_size := 40
	var shore_wave_frequency := 0.05
	var shore_wave_depth := 6
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			var coord := Vector3i(x - MAP_SIZE_X/2, 0, y - MAP_SIZE_Y/2)
			if y < MAP_SIZE_Y - water_area_y_size - sin(x * shore_wave_frequency) * shore_wave_depth:
				ground_grid.set_cell_item(coord, 0)
				
				var noise_value := rock_placement_noise.get_noise_2d(x, y)
				if noise_value > 0.2:
					var noise_value2 := rock_placement_noise.get_noise_2d(y*2, x*2)
					if noise_value2 > -0.1:
						grid.set_cell_item(coord, 1)
					else:
						grid.set_cell_item(coord, 2)
					Events.solid_cell_placed.emit(Globals.truncate_vec3i(coord))
			else:
				ground_grid.set_cell_item(coord, 1)

	
	_create_rocks()
	_create_rivers()
	_create_grass()
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			var final_x: int = x - MAP_SIZE_X/2
			var final_y: int = y - MAP_SIZE_Y/2
			var cell := ground_grid.get_cell_item(Vector3i(final_x, 0, final_y))
			if cell == GroundCells.Water or cell == GroundCells.RiverWater:
				PathFinder.set_coordinate_invalid(Vector2i(final_x, final_y))

var width_shapes: Dictionary = {
	1: [Vector2i(0, 0)],
	2: [
		Vector2i(0, -1),
		Vector2i(1, 0),
		Vector2i(0, 1),
		Vector2i(-1, 1),
		],
	3: [
		Vector2i(-1, -1),
		Vector2i(1, -1),
		Vector2i(1, 1),
		Vector2i(-1, 1),
	]
}

func _create_rocks() -> void:
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			var coord := Vector3i(x - MAP_SIZE_X/2, 0, y - MAP_SIZE_Y/2)
			var noise_value := rock_placement_noise.get_noise_2d(x, y)
			if noise_value > 0.2:
				var noise_value2 := rock_placement_noise.get_noise_2d(y*2, x*2)
				grid.set_cell_item(coord, 1)
				var entity: Entity = Entity.from_definition(load("res://src/entities/definitions/terrain/rock.tres"))
				Events.request_entity_add.emit(entity)
				var container: ComponentContainer = entity.component_container
				WorldPositionComponent.set_world_position(entity, coordinate_to_global_position(Globals.truncate_vec3i(coord)))
				var terrain: TerrainComponent = container.get_by_id(Components.Id.Terrain)
				if noise_value2 > -0.1:
					terrain.mesh_id = AboveGroundCells.Rock
				else:
					terrain.mesh_id = AboveGroundCells.Rock2


func _create_rivers() -> void:
	var rivers: int = 1

	for i in rivers:
		var river_start := Vector2i(randi_range(-MAP_SIZE_X/2, MAP_SIZE_X/2), MAP_SIZE_Y/2-1)
		var river_start_angle := 3*PI/2
		var river_coordinates: Array[Vector2i]
		_create_river(river_start, 3, river_start_angle, river_coordinates)
		for coordinate in river_coordinates:
			var grid_coord := Globals.extend_vec2i(coordinate)
			var cell_item := ground_grid.get_cell_item(grid_coord)
			if cell_item != GroundCells.Water and cell_item != -1:
				ground_grid.set_cell_item(grid_coord, GroundCells.RiverWater)
				PathFinder.set_coordinate_invalid(Globals.truncate_vec3i(grid_coord))
				grid.set_cell_item(grid_coord, -1)
			
				var surrounding := PathFinder.get_surrounding_coordinates(coordinate, true)
				for surrounding_coord in surrounding:
					var surrounding_cell := ground_grid.get_cell_item(Globals.extend_vec2i(surrounding_coord))
					
					if surrounding_cell != GroundCells.RiverWater and surrounding_cell != GroundCells.Water and surrounding_cell != -1:
						ground_grid.set_cell_item(Globals.extend_vec2i(surrounding_coord), GroundCells.RiverBank)
						PathFinder.set_coordinate_invalid(surrounding_coord)

func _create_river(starting_coordinate: Vector2i, max_branches_left: int, starting_angle: float, array_to_fill: Array[Vector2i]) -> void:
	#region Invariables
	var river_length: int = 300
	var starting_point := coordinate_to_global_position(starting_coordinate)
	var step_length: float = 1.0
	var direction_range := PI/24
	var branching_chance := 0.02
	var branch_angle_range := PI/8
	var starting_width: float = 1.0
	var width_variance: float = 0.1
	var max_width: float = 3.0
	#endregion
	
	#region Variables
	var current_angle := starting_angle
	var current_point := starting_point
	var current_width := starting_width
	#endregion
	
	for i in river_length:
		current_width += randf_range(-width_variance/2, width_variance)
		current_width = clampf(current_width, 0, max_width)
		
		var current_coord := global_position_to_coordinate(current_point)
		
		# Terminate if hit existing water body or out of bounds
		var current_cell := ground_grid.get_cell_item(Globals.extend_vec2i(current_coord))
		#if current_cell == 1 or current_cell == -1:
		if current_cell == -1:
			return
		
		for w in int(maxi(current_width, 1)):
			var shapes: Array = width_shapes[w+1]
			for shape_coord: Vector2i in shapes:
				var final_pos := current_coord + shape_coord
				#ground_grid.set_cell_item(Globals.extend_vec2i(final_pos), 1)
				array_to_fill.append(final_pos)
		var angle_vec := Vector2.from_angle(current_angle) * step_length
		current_point += Globals.extend_vec2(angle_vec)
		current_angle += randf_range(-direction_range, direction_range)
		
		if max_branches_left > 0 and randf() < branching_chance:
			max_branches_left -= 1
			var new_angle := current_angle + randf_range(-branch_angle_range/2, branch_angle_range)
			_create_river(current_coord, max_branches_left, new_angle, array_to_fill)
		

func add_map_entity(coordinate: Vector2i, entity: Node3D) -> void:
	if not map_entities.has(coordinate):
		map_entities[coordinate] = []
	
	if not map_entities[coordinate].has(entity):
		map_entities[coordinate].append(entity)

func remove_map_entity(coordinate: Vector2i, entity: Node3D) -> void:
	if map_entities.has(coordinate):
		map_entities[coordinate].erase(entity)

func get_map_entities(coordinate: Vector2i, items_only: bool = false) -> Array[Node3D]:
	var result: Array[Node3D]
	if map_entities.has(coordinate):
		result.assign(map_entities[coordinate])
		result = result.filter(func(entity: Node3D) -> bool:
			if items_only:
				return entity is Entity
			return true
			)
	return result

func is_coordinate_occupied(coordinate: Vector2i) -> bool:
	if not map_entities.has(coordinate):
		return false
	
	var entities := map_entities[coordinate] as Array
	
	for entity in entities as Array[Entity]:
		if entity is Entity:
			var container := entity.component_container
			if container.has_component(Components.Id.Solid):
				return true
			
			var constructable: ConstructableComponent = container.get_by_id(Components.Id.Constructable)
			if constructable and constructable.solid_when_started:
				return true
	
	return false
	

func _ready() -> void:
	prepare_blueprint_grid()
	
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
		
	Events.item_removed_from_ground.connect(func(item: Entity) -> void:
			var coordinate := global_position_to_coordinate(item.global_position)
			remove_map_entity(coordinate, item)
	)
	
	Events.ui_action_selected.connect(_handle_ui_action_selection)
	Events.ui.action_cleared.connect(func() -> void:
		selected_ui_action = null
		)
	
	Events.map_ready.emit(self)

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
			var selectable: SelectableComponent = entity_to_select.component_container.get_by_id(Components.Id.Selectable)
			if selectable:
				selectable.selected = true
	
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
			if entity is Entity:
				var container: ComponentContainer = entity.component_container
				var constructable_component: ConstructableComponent = container.get_by_id(Components.Id.Constructable)
				if constructable_component:
					if constructable_component.can_be_dismantled and not constructable_component.reserved_for_dismantling:
						var selected_action: UiAction.Dismantle = selected_ui_action
						var has_all := true
						for tag in selected_action.target_tag_filters:
							var tag_component: TagComponent = entity.component_container.get_by_id(Components.Id.Tag)
							if not tag_component or not tag_component.has_tag(tag):
								has_all = false
								break
						
						if has_all:
							Events.dismantle_issued.emit(entity)

func _zone_add_tiles(coordinates: Array[Vector2i]) -> void:
	var zone := (selected_ui_action as UiAction.ZoneAddTiles).zone
	zone.add_coordinates(coordinates)

func _create_placement_juice(entity: Entity, index: int) -> void:
	var container: ComponentContainer = entity.component_container
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
		if node.has_method("set_blueprint"):
			node.set_blueprint(true)
		
		if node.has_method("set_active"):
			node.set_active(false)
		removed_component = scene_component
	
	if not node:
		return
	
	await get_tree().create_timer(float(index) * 0.02).timeout
	add_child(node)
	var pos: Vector3 = container.get_by_id(Components.Id.WorldPosition).current_position + position_correction
	node.global_position = pos + Vector3(0, 1.0, 0)
	$PlacementJuicePlayer.pitch_scale = 1 + randf_range(-0.1, 0.1)
	$PlacementJuicePlayer.play()
	await create_tween().tween_property(node, "global_position", pos, 0.5).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_BOUNCE).finished
	node.queue_free()
	if not container: return
	container.add_component(removed_component)

func _place_blueprint(coordinates: Array[Vector2i]) -> void:
	var item := (selected_ui_action as UiAction.Build).item
	var _index: int = 0
	for tile_position in coordinates:
		if not is_coordinate_occupied(tile_position):
			var blueprint: Entity = Entity.from_definition(item)
			Events.request_entity_add.emit(blueprint)
			WorldPositionComponent.set_world_position(blueprint, coordinate_to_global_position(tile_position))
			blueprint.component_container.add_component(BlueprintComponent.new())
			blueprint.component_container.add_component(TagComponent.new()).add_tag(TagComponent.Tag.PlayerMade)
			Events.blueprint_placed.emit(tile_position, blueprint)
			_create_placement_juice(blueprint, _index)
			_index += 1

func _cancel_blueprint(coordinates: Array[Vector2i]) -> void:
	for coordinate in coordinates:
		var entities := get_map_entities(coordinate)
		for entity: Entity in entities:
			if entity.component_container.has_component(Components.Id.Blueprint):
				entity.delete()

func _terrain_placed(coordinate: Vector2i, mesh_id: MapMeshes.Id, blueprint: bool) -> void:
	var grid_map: GridMap = grid if not blueprint else blueprint_grid
	grid_map.set_cell_item(Globals.extend_vec2i(coordinate), mesh_id)

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

func _serialize_grid_map(items: Array, map: GridMap) -> Dictionary:
	var dict: Dictionary
	for item: int in items:
		dict[item] = map.get_used_cells_by_item(item).map(func(vec: Vector3i) -> Dictionary:
			return {
				x = vec.x, y = vec.y, z = vec.z
			}
			)
	
	return {
		map_cells_by_item = dict
	}

func serialize() -> Dictionary:
	var grids: Dictionary = {}
	
	for key: String in grid_definitions.keys():
		var definition: Dictionary = grid_definitions[key]
		grids[key] = _serialize_grid_map(definition.items, definition.map) 
	
	return {
		grids = grids
	}

func deserialize(dict: Dictionary) -> void:
	clear_everything()
	
	for grid_definition_key: String in dict["grids"].keys():
		var grid_map: GridMap = grid_definitions[grid_definition_key].map
		var grid_dict: Dictionary = dict["grids"][grid_definition_key]
		for cell_id: int in grid_dict["map_cells_by_item"].keys():
			for coord_dict: Dictionary in grid_dict["map_cells_by_item"][cell_id]:
				var coord: Vector3i = Vector3i(coord_dict["x"], coord_dict["y"], coord_dict["z"])
				grid_map.set_cell_item(coord, cell_id)
				
				## TODO: Handle grid map collisions properly
				## TODO 2: Actually should just serialize PathFinder probably
				## Would be MUCH better
				#if grid_definition_key == "grid":
					#PathFinder.set_coordinate_invalid(Globals.truncate_vec3i(coord))
				#
				#if grid_definition_key == "ground_grid":
					#if cell_id == GroundCells.Water or cell_id == GroundCells.RiverWater:
						#PathFinder.set_coordinate_invalid(Globals.truncate_vec3i(coord))

	_create_grass()
