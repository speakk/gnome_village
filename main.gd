extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")

func _ready() -> void:
	var test_divider := 2
	var map_size_real_x := MainMap.MAP_SIZE_X * 24 / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y * 24 / test_divider
	
	for i in 20:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(Items.Id.Tree)
			item_on_ground.global_position = quantized_position
			add_child(item_on_ground)
	
	await get_tree().physics_frame

	var item_types: Array[Items.Id] = [Items.Id.Wood, Items.Id.Stone]

	for i in 140:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_types.pick_random())
			item_on_ground.global_position = quantized_position
			add_child(item_on_ground)
	
	var settlers_to_place := 10
	var attempts := 400
	for i in attempts:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var settler := SETTLER.instantiate()
			settler.global_position = quantized_position
			add_child(settler)
			settlers_to_place -= 1
			if settlers_to_place <= 0:
				break

var debug_visuals := false

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("debug_toggle"):
		debug_visuals = not debug_visuals
		Events.debug_visuals_set.emit(debug_visuals)
