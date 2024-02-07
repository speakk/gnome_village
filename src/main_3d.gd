extends Node3D

@onready var SETTLER := preload("res://src/settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

@onready var main_map: MainMap3D = $MainMap3d as MainMap3D

@export var daylight_amount: Curve
@export var yellow_light_amount: Curve

const TEST_TREES = 1
const TEST_RESOURCES = 2
const TEST_SETTLERS = 5
const DECAL_AMOUNT = 2

func _ready() -> void:
	Events.load_game_called.connect(func(save_dict: Dictionary) -> void: load_save(save_dict))
	Events.save_game_called.connect(func(save_dict: Dictionary) -> void: save(save_dict))
	
	#Events.current_time_changed.connect(_current_time_changed)
	
	var test_divider := 1
	var map_size_real_x := MainMap3D.MAP_SIZE_X / test_divider
	var map_size_real_y := MainMap3D.MAP_SIZE_Y / test_divider
	
	print("Now spawning entities")
	
	#for i in TEST_TREES:
		#var random_position := Vector3(randf_range(0, map_size_real_x), 0, randf_range(0, map_size_real_y))
		#var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		#var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		#if not PathFinder.is_position_solid(grid_position):
			#var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			#%Entities.add_child(item_on_ground)
			#item_on_ground.global_position = quantized_position
			#item_on_ground.initialize(Items.Id.Tree)
			#
	#
	#await get_tree().physics_frame
#
	#var item_types: Array[Items.Id] = [Items.Id.Wood, Items.Id.Stone]
#
	#for i in TEST_RESOURCES:
		#var random_position := Vector3(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y), 0.0)
		#var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		#var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		#if not PathFinder.is_position_solid(grid_position):
			#var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			#%Entities.add_child(item_on_ground)
			#item_on_ground.global_position = quantized_position
			#item_on_ground.initialize(item_types.pick_random())
	
	var settlers_to_place := TEST_SETTLERS
	var attempts := 400
	for i in attempts:
		var grid_position := Vector2i(randi_range(0, map_size_real_x), randi_range(0, map_size_real_y))
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var settler := SETTLER.instantiate()
			%Entities.add_child(settler)
			settler.global_position = quantized_position
			
			print("Settler placed at", settler.global_position)
			
			settlers_to_place -= 1
			if settlers_to_place <= 0:
				break
	
	var DECAL := preload("res://src/misc/Decal.tscn")
	for i in DECAL_AMOUNT:
		var decal := DECAL.instantiate()
		var grid_position := Globals.get_map().get_random_coordinate()
		var new_position := Globals.get_map().coordinate_to_global_position(grid_position)
		decal.global_position = new_position + Vector3(randf_range(-6, 6), randf_range(-6, 6), 0)
		%Entities.add_child(decal)
		

var debug_visuals := false

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("debug_toggle"):
		debug_visuals = not debug_visuals
		Events.debug_visuals_set.emit(debug_visuals)

	if Input.is_action_just_pressed("game_speed_1"):
		Engine.time_scale = 1.0
	
	if Input.is_action_just_pressed("game_speed_2"):
		Engine.time_scale = 2.0
	
	if Input.is_action_just_pressed("game_speed_3"):
		Engine.time_scale = 5.0
	
# For now this is okay, but eventually Entities and
# TaskHandler ought to have their own save methods
@onready var containers := [
	{
		data_name = "entities",
		node = %Entities,
	},
	{
		data_name = "tasks",
		node = %TaskHandler.get_node("Tasks"),
	},
] as Array[Dictionary]

func load_save(data: Dictionary) -> void:
	main_map.prepare_for_load()
	PathFinder.prepare_for_load()
	
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
#func _current_time_changed(new_time: float) -> void:
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
