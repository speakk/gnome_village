extends Node3D

@onready var SETTLER := load("res://src/entities/scenes/settler/settler.tscn")
@onready var ENTITY := load("res://src/entities/entity/Entity.tscn")

@onready var main_map: MainMap = $MainMap as MainMap
@onready var entities: Node3D = %Entities

@onready var sky: DayNightCycleSky = $sky

@export var daylight_amount: Curve
@export var yellow_light_amount: Curve

const TEST_TREES = 100
const TEST_RESOURCES = 400
const TEST_SETTLERS = 40
const DECAL_AMOUNT = 800

func _ready() -> void:
	Events.load_game_called.connect(func(save_dict: Dictionary) -> void: load_save(save_dict))
	Events.save_game_called.connect(func(save_dict: Dictionary) -> void: save(save_dict))
	
	Events.request_entity_add.connect(func(entity: Node) -> void:
		entities.add_child(entity)
	)
	
	Events.current_time_changed.connect(_current_time_changed)
	
	create_world()

var debug_visuals := false

func _clear_entities() -> void:
	for container in containers:
		for entity in container["node"].get_children() as Array[Node]:
			entity.queue_free()

func create_world() -> void:
	_clear_entities()
	#PathFinder.prepare_for_load()
	main_map.prepare_for_load(true)
	
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

func load_save(data: Dictionary) -> void:
	PathFinder.prepare_for_load()
	main_map.prepare_for_load(true)
	
	await get_tree().physics_frame
	
	for container in containers:
		for entity in container["node"].get_children() as Array[Node]:
			entity.queue_free()
		
		var entity_ids: Array[int]
		entity_ids.assign(data["main_data"][container["data_name"]])
		
		for entity_id in entity_ids:
			var entity: Variant = SaveSystem.get_saved_entity(entity_id)
			print("Adding child right?", entity)
			if not entity is Resource:
				container["node"].add_child(entity)
			#SaveSystem.load_entity(entity)

func save(save_dict: Dictionary) -> void:
	var main_data: Dictionary = {}
	main_data["entities"] = []
	main_data["tasks"] = []
	
	for container in containers:
		for entity in container["node"].get_children() as Array[Node]:
			if entity.has_method("save"):
				var entity_id := SaveSystem.save_entity(entity)
				main_data[container["data_name"]].append(entity_id)
			else:
				pass
				#push_warning("Entity did not have save method defined: ", entity)
	
	save_dict["main_data"] = main_data
#

func create_ground_entities() -> void:
	for iter_x in MainMap.MAP_SIZE_X:
		for iter_y in MainMap.MAP_SIZE_Y:
			var x: int = iter_x - MainMap.MAP_SIZE_X/2
			var y: int = iter_y - MainMap.MAP_SIZE_Y/2
			var mesh_id: int = main_map.ground_grid.get_cell_item(Vector3i(x, 0, y))
			if mesh_id == 0:
				var entity: Entity = ENTITY.instantiate()
				entity.show_amount_number = false
				main_map.add_map_entity(Vector2i(x, y), entity)
				entities.add_child(entity)
				entity.hide()
				entity.process_mode = Node.PROCESS_MODE_DISABLED
				entity.component_container.component_owner = entity
				var inventory: InventoryComponent = InventoryComponent.new()
				inventory.pre_filled = [ItemRequirement.new(preload("res://src/entities/definitions/water.tres"), 100)]
				inventory.items_can_be_picked = false
				entity.component_container.add_component(inventory)
				var world_pos: WorldPositionComponent = WorldPositionComponent.new()
				world_pos = entity.component_container.add_component(world_pos)
				world_pos.current_position = main_map.coordinate_to_global_position(Vector2i(x, y))


func _current_time_changed(new_time: float) -> void:
	#sky.set_time_of_day(new_time)
	sky.time_of_day_setup = new_time
	#var daylight_sampled := daylight_amount.sample(new_time)
	#var red_green := daylight_sampled
	#var yellow_amount := yellow_light_amount.sample(new_time)
	##$CanvasModulate.color = Color(red_green, red_green - yellow_amount * 0.1, 1.0 - yellow_amount * 0.4)
	#var shaderNode := %ShadowSpriteShader as Sprite2D
	#shaderNode.material.set_shader_parameter("shadow_angle", - new_time * 360.0 * 2)
	#shaderNode.material.set_shader_parameter("shadow_length", 200 - daylight_sampled * 190)
	#shaderNode.material.set_shader_parameter("shadow_color", Color(Color.BLACK, maxf(0, daylight_sampled - 0.3)))
	#
	#var daylightNode := %DayLightSpriteShader as Sprite2D
	#daylightNode.material.set_shader_parameter("daylight_amount", daylight_sampled)
	#daylightNode.material.set_shader_parameter("yellow_amount", yellow_amount)
	#
	##shaderNode.material.set("shader_param/angle", -sin(new_time) * 360.0)
	##print("New time", new_time)
