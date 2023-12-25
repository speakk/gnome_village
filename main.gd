extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var ITEM_ON_GROUND := preload("res://items/ItemOnGround.tscn")

func _ready() -> void:
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X * 24 / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y * 24 / test_divider
	
	for i in 2:
		var settler := SETTLER.instantiate()
		settler.global_position = Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		add_child(settler)

	var item_types: Array[Items.Id] = [Items.Id.Wood]

	for i in 30:
		var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_types.pick_random())
		item_on_ground.global_position = Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		add_child(item_on_ground)
