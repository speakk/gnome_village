class_name Main extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")

func _ready() -> void:
	Events.load_game_called.connect(func(save_dict: Dictionary) -> void: load_save(save_dict))
	Events.save_game_called.connect(func(save_dict: Dictionary) -> void: save(save_dict))
	
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X * 24 / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y * 24 / test_divider
	
	for i in 20:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			item_on_ground.global_position = quantized_position
			%Entities.add_child(item_on_ground)
			item_on_ground.initialize(Items.Id.Tree)
	
	await get_tree().physics_frame

	var item_types: Array[Items.Id] = [Items.Id.Wood, Items.Id.Stone]

	for i in 180:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			item_on_ground.global_position = quantized_position
			%Entities.add_child(item_on_ground)
			item_on_ground.initialize(item_types.pick_random())
	
	var settlers_to_place := 20
	var attempts := 400
	for i in attempts:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var settler := SETTLER.instantiate()
			settler.global_position = quantized_position
			%Entities.add_child(settler)
			settlers_to_place -= 1
			if settlers_to_place <= 0:
				break

var debug_visuals := false

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("debug_toggle"):
		debug_visuals = not debug_visuals
		Events.debug_visuals_set.emit(debug_visuals)

func load_save(data: Dictionary) -> void:
	for entity in %Entities.get_children():
		entity.queue_free()
	
	var entities: Dictionary = data["main_data"]["entities"]
	
	for entity_dict in entities.values() as Array[Dictionary]:
		var new_object: Variant = load(entity_dict["filename"]).instantiate()
		%Entities.add_child(new_object)
		new_object.load_save(entity_dict)
		new_object.persistent.set_save_id(entity_dict["save_id"])

func save(save_dict: Dictionary) -> void:
	var main_data: Dictionary = {}
	main_data["entities"] = {}
	
	for entity in %Entities.get_children():
		if entity.has_method("save"):
			var entity_dict: Dictionary = entity.save()
			SaveSystem.enrich_save_data(entity, entity_dict)
			main_data["entities"][entity_dict["save_id"]] = entity_dict
		else:
			push_warning("Entity did not have save method defined: ", entity)
	
	#print("Saving main data: ", main_data)
	
	save_dict["main_data"] = main_data
