extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://items/ItemOnGround.tscn")

func _ready() -> void:
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X * 24 / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y * 24 / test_divider
	
	var item_types: Array[Items.Id] = [Items.Id.Wood]

	for i in 30:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(Items.Id.Tree)
			item_on_ground.global_position = quantized_position
			add_child(item_on_ground)
	
	await get_tree().physics_frame

	for i in 30:
		var random_position := Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		var grid_position := Globals.get_map().global_position_to_coordinate(random_position)
		var quantized_position := Globals.get_map().coordinate_to_global_position(grid_position)
		if not PathFinder.is_position_solid(grid_position):
			var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(Items.Id.Wood)
			item_on_ground.global_position = quantized_position
			add_child(item_on_ground)
	
	var settlers_to_place := 4
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

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				$MainCamera.zoom += Vector2(0.5, 0.5)
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				$MainCamera.zoom -= Vector2(0.5, 0.5)
		
		$MainCamera.zoom = $MainCamera.zoom.clamp(Vector2(0.25, 0.25), Vector2(4.0, 4.0))
	
