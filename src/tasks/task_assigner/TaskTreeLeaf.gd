extends Node

class_name TaskTreeLeaf

var task: Task

var task_name: String

func clean_up() -> void:
	task.queue_free()

func set_task(_task: Task) -> void:
	assert(_task)
	task = _task
	add_child(task)
	task_name = task.task_name
