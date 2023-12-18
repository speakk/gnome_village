extends Node

@onready var BLUEPRINT: PackedScene = preload("res://task_assigners/blueprint.tscn")

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)

#func _process(delta: float) -> void:
	#for child in get_children():
		#child = child as BeehaveTree
		#child.tick()
		#print("Ticking")

func _blueprint_placed(target_tile: Vector2i, blueprint: Blueprint) -> void:
	var blueprint_assigner: BluePrintTaskAssigner = (BLUEPRINT.instantiate() as BluePrintTaskAssigner).initialize(target_tile, blueprint)
	add_child(blueprint_assigner)
