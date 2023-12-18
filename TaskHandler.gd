extends Node

@onready var BLUEPRINT: PackedScene = preload("res://task_assigners/blueprint.tscn")

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)
	Events.task_finished.connect(_task_finished)
	Events.task_assigner_finished.connect(_task_assigner_finished)

#func _process(delta: float) -> void:
	#for child in get_children():
		#child = child as BeehaveTree
		#child.tick()
		#print("Ticking")

func _blueprint_placed(target_tile: Vector2i, blueprint: Blueprint) -> void:
	var blueprint_assigner: BluePrintTaskAssigner = (BLUEPRINT.instantiate() as BluePrintTaskAssigner).initialize(target_tile, blueprint)
	add_child(blueprint_assigner)

func _task_finished(task: Task) -> void:
	task.disable()
	
func _task_assigner_finished(task_assigner: BeehaveTree) -> void:
	print("Finished...")
	task_assigner.disable()
	task_assigner.queue_free()
