extends Node3D

@onready var SETTLER := preload("res://src/settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

@onready var main_map: MainMap3D = $MainMap3d as MainMap3D
@onready var entities: Node3D = %Entities

@onready var sky: DayNightCycleSky = $sky

@export var test_tree: Item

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
	
	var test_divider := 1
	var map_size_real_x := MainMap3D.MAP_SIZE_X / test_divider
	var map_size_real_y := MainMap3D.MAP_SIZE_Y / test_divider
	
	print("Now spawning entities")
	
	#for i in TEST_TREES:
		#var grid_position := Globals.get_map().get_random_coordinate()
		#var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		#if not PathFinder.is_position_solid(grid_position):
			#var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			#%Entities.add_child(item_on_ground)
			#item_on_ground.initialize(Items.Id.Tree)
			#WorldPositionComponent.set_world_position(item_on_ground, quantized_position)
	
	for i in TEST_TREES:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var new_grows_in_entity := PlantComponent.create_growth_spot(quantized_position)
			
			var new_tree: Item = test_tree.duplicate(true)
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround) 
			%Entities.add_child(item_on_ground)
			item_on_ground.item = new_tree
			WorldPositionComponent.set_world_position(item_on_ground, quantized_position)
			item_on_ground.component_container.get_by_id(Components.Id.Plant).grows_in = new_grows_in_entity.component_container.get_by_id(Components.Id.GrowthSpot)
			item_on_ground.component_container.get_by_id(Components.Id.Plant).current_growth_stage_index = randi_range(0, 3)
			
	await get_tree().physics_frame
#
	var item_types: Array[Items.Id] = [Items.Id.Wood, Items.Id.Stone, Items.Id.Potato]
#
	for i in TEST_RESOURCES:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			%Entities.add_child(item_on_ground)
			item_on_ground.item = Items.get_by_id(item_types.pick_random())
			WorldPositionComponent.set_world_position(item_on_ground, quantized_position)
			
	
	var settlers_to_place := TEST_SETTLERS
	var attempts := 400
	var settler_radius_modifier := 0.3
	for i in attempts:
		var grid_position := Globals.get_map().get_random_coordinate() * settler_radius_modifier
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var settler := SETTLER.instantiate()
			%Entities.add_child(settler)
			WorldPositionComponent.set_world_position(settler, quantized_position)
			
			settlers_to_place -= 1
			if settlers_to_place <= 0:
				break
	
	#create_ground_entities()
	
	var decal_items: Array[Items.Id] = [Items.Id.Flower1, Items.Id.Flower2]
	for i in DECAL_AMOUNT:
		var grid_position := Globals.get_map().get_random_coordinate()
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			%Entities.add_child(item_on_ground)
			item_on_ground.item = Items.get_by_id(decal_items.pick_random())
			WorldPositionComponent.set_world_position(item_on_ground, quantized_position)
			item_on_ground.rotate_y(randf_range(0, PI*2))

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
		Engine.time_scale = 9.0
	
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
	main_map.prepare_for_load()
	
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
	for iter_x in MainMap3D.MAP_SIZE_X:
		for iter_y in MainMap3D.MAP_SIZE_Y:
			var x: int = iter_x - MainMap3D.MAP_SIZE_X/2
			var y: int = iter_y - MainMap3D.MAP_SIZE_Y/2
			var mesh_id: int = main_map.ground_grid.get_cell_item(Vector3i(x, 0, y))
			if mesh_id == 0:
				var entity: ItemOnGround = ITEM_ON_GROUND.instantiate()
				entity.show_amount_number = false
				main_map.add_map_entity(Vector2i(x, y), entity)
				entities.add_child(entity)
				entity.hide()
				entity.process_mode = Node.PROCESS_MODE_DISABLED
				entity.component_container.component_owner = entity
				var inventory: InventoryComponent = InventoryComponent.new()
				inventory.pre_filled = [ItemRequirement.new(Items.Id.Water, 100)]
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
