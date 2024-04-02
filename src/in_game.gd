class_name InGame extends Node3D

@onready var SETTLER := load("res://src/entities/scenes/settler/settler.tscn")
@onready var ENTITY := load("res://src/entities/entity/Entity.tscn")

@onready var main_map: MainMap = $MainMap as MainMap
@onready var entities: Node3D = %Entities
@onready var save_system: SaveSystem = $SaveSystem

@onready var sky: DayNightCycleSky = $sky

@export var daylight_amount: Curve
@export var yellow_light_amount: Curve

const TEST_TREES = 100
const TEST_RESOURCES = 400
const TEST_SETTLERS = 10
const DECAL_AMOUNT = 800

func _ready() -> void:
	Events.request_entity_add.connect(func(entity: Node) -> void:
		entities.add_child(entity)
	)
	
	Events.load_started.connect(func() -> void:
		for child in %Entities.get_children():
			child.queue_free()
		)
	
	Events.current_time_changed.connect(_current_time_changed)
	
	var save_method := SaveSystem.SaveMethod.new("in_game", _save_callable, _load_callable)
	SaveSystem.register_save_method(save_method)
	

var debug_visuals := false

func _clear_entities() -> void:
	for container in containers:
		for entity in container["node"].get_children() as Array[Node]:
			entity.queue_free()

func new_game() -> void:
	create_world()

func create_world() -> void:
	_clear_entities()
	#PathFinder.prepare_for_load()
	main_map.create_world()
	
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y / test_divider
	
	print("Now spawning entities")
	
	for i in TEST_TREES:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var new_grows_in_entity := PlantComponent.create_growth_spot(quantized_position)
			var entity: Entity = Entity.from_definition(load("res://src/entities/definitions/plants/oak_tree.tres"))
			%Entities.add_child(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			entity.component_container.get_by_id(Components.Id.Plant).grows_in = new_grows_in_entity.component_container.get_by_id(Components.Id.GrowthSpot)
			entity.component_container.get_by_id(Components.Id.Plant).current_growth_stage_index = randi_range(0, 3)
			entity.name = "oak_tree"
			
	await get_tree().physics_frame
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
			%Entities.add_child(entity)
			WorldPositionComponent.set_world_position(entity, quantized_position)
			
	
	var settlers_to_place := TEST_SETTLERS
	var attempts := 400
	var settler_radius_modifier := 0.3
	for i in attempts:
		var grid_position := Globals.get_map().get_random_coordinate() * settler_radius_modifier
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var settler: Settler = Entity.from_definition(load("res://src/entities/definitions/settler.tres"))
			%Entities.add_child(settler)
			WorldPositionComponent.set_world_position(settler, quantized_position)
			
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
			%Entities.add_child(entity)
			
			WorldPositionComponent.set_world_position(entity, quantized_position)
			entity.rotate_y(randf_range(0, PI*2))

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

# For now this is okay, but eventually Entities and
# TaskHandler ought to have their own save methods
@onready var containers := [
	{
		data_name = "entities",
		node = %Entities,
	},
	#{
		#data_name = "tasks",
		#node = %TaskManager.get_node("Tasks"),
	#},
] as Array[Dictionary]

func _save_callable() -> Dictionary:
	var entities := get_tree().get_nodes_in_group("entity")
	var entity_dicts: Array[Dictionary]
	for entity: Entity in entities as Array[Entity]:
		var save_dict := entity.serialize()
		entity_dicts.append(save_dict)
	
	var saved_map := main_map.serialize()
	var task_manager: Dictionary = TaskManager.serialize()
	
	return {
		entities = entity_dicts,
		map = saved_map,
		task_manager = task_manager
	}
	
func _load_callable(save_dict: Dictionary) -> void:
	main_map.deserialize(save_dict["map"])
	
	for entity_dict: Dictionary in save_dict["entities"]:
		Entity.static_deserialize(%Entities, entity_dict)
	
	TaskManager.deserialize(save_dict["task_manager"])

func quick_load() -> void:
	save_system.quick_load()

func _current_time_changed(new_time: float) -> void:
	sky.time_of_day_setup = new_time
