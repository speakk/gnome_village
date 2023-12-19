extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")

func _ready() -> void:
	#pass
	for i in 100:
		var settler := SETTLER.instantiate()
		settler.global_position = Vector2(randf_range(0, 1280), randf_range(0, 640))
		add_child(settler)
