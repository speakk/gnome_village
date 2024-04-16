class_name InGame extends Node3D

@onready var SETTLER := load("res://src/entities/scenes/settler/settler.tscn")

@onready var main_map: MainMap = $MainMap as MainMap
@onready var entity_scenes: Node3D = %EntityScenes
@onready var entity_handler: EntityHandler = %EntityHandler
@onready var save_system: SaveSystem = $SaveSystem

@onready var sky: DayNightCycleSky = $sky

@export var daylight_amount: Curve
@export var yellow_light_amount: Curve

const TEST_TREES = 100
const TEST_RESOURCES = 400
const TEST_SETTLERS = 10
const DECAL_AMOUNT = 800

func _ready() -> void:
	Events.request_entity_scene_add.connect(func(entity_scene: Node3D) -> void:
		entity_scenes.add_child(entity_scene)
	)
	
	Events.load_started.connect(func() -> void:
		for child in %EntityScenes.get_children():
			child.queue_free()
		)
	
	Events.current_time_changed.connect(_current_time_changed)
	
	var save_method := SaveSystem.SaveMethod.new("in_game", _save_callable, _load_callable)
	SaveSystem.register_save_method(save_method)
	

var debug_visuals := false

func _clear_entity_scenes() -> void:
	for entity in %EntityScenes.get_children():
		entity.queue_free()

func new_game() -> void:
	create_world()

func create_world() -> void:
	Events.world_creation.begin.emit()
	_clear_entity_scenes()
	
	await main_map.create_world()
	
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y / test_divider
	
	print("Now spawning entities")
	
	Events.world_creation.entities.emit()
	await get_tree().process_frame
	
	for i in TEST_TREES:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var new_grows_in_entity := PlantComponent.create_growth_spot(quantized_position)
			var entity: Entity = Entity.from_definition(load("res://src/entities/definitions/plants/oak_tree.tres"))
			entity_handler.add_entity(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			entity.component_container.get_by_id(Components.Id.Plant).grows_in = new_grows_in_entity.component_container.get_by_id(Components.Id.GrowthSpot)
			entity.component_container.get_by_id(Components.Id.Plant).current_growth_stage_index = randi_range(0, 3)
			
#
	var resources: Array[EntityDefinition] = [
		preload("res://src/entities/definitions/wood.tres"),
		preload("res://src/entities/definitions/stone.tres"),
		preload("res://src/entities/definitions/food/potato.tres"),
		]
#
	for i in TEST_RESOURCES:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var entity: Entity = Entity.from_definition(resources.pick_random())
			entity_handler.add_entity(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			
	
	var settlers_to_place := TEST_SETTLERS
	var attempts := 400
	var settler_radius_modifier := 0.3
	for i in attempts:
		var grid_position := Globals.get_map().get_random_coordinate() * settler_radius_modifier
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var entity: Entity = Entity.from_definition(load("res://src/entities/definitions/settler.tres"))
			entity_handler.add_entity(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			
			settlers_to_place -= 1
			if settlers_to_place <= 0:
				break
	
	var decal_items: Array[EntityDefinition] = [
		preload("res://src/entities/definitions/foliage/flower_1.tres"),
		preload("res://src/entities/definitions/foliage/flower_2.tres")
		]
	for i in DECAL_AMOUNT:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var entity: Entity = Entity.from_definition(decal_items.pick_random())
			entity_handler.add_entity(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			entity.component_container.get_by_id(Components.Id.Scene).get_scene().rotate_y(randf_range(0, PI*2))

	Events.world_creation.finished.emit()
	
func _process(delta: float) -> void:
	if Input.is_action_just_pressed("debug_toggle"):
		debug_visuals = not debug_visuals
		Events.debug_visuals_set.emit(debug_visuals)

	if Input.is_action_just_pressed("game_speed_1"):
		Engine.time_scale = 1.0
	
	if Input.is_action_just_pressed("game_speed_2"):
		Engine.time_scale = 2.0
	
	if Input.is_action_just_pressed("game_speed_3"):
		Engine.time_scale = 9.0
	
	if Input.is_action_just_pressed("reload_map"):
		create_world()

func _save_callable() -> Dictionary:	
	var saved_map := main_map.serialize()
	var entity_handler: Dictionary = entity_handler.serialize()
	var task_manager: Dictionary = TaskManager.serialize()
	var path_finder: Dictionary = PathFinder.serialize()
	
	return {
		entity_handler = entity_handler,
		map = saved_map,
		task_manager = task_manager,
		path_finder = path_finder
	}
	
func _load_callable(save_dict: Dictionary) -> void:
	main_map.deserialize(save_dict["map"])
	entity_handler.deserialize(save_dict["entity_handler"])
	TaskManager.deserialize(save_dict["task_manager"])
	PathFinder.deserialize(save_dict["path_finder"])

func quick_load() -> void:
	save_system.quick_load()

func _current_time_changed(new_time: float) -> void:
	sky.time_of_day_setup = new_time
