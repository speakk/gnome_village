extends Node

class_name TaskTreeLeaf

var task: Task

func clean_up() -> void:
	task.queue_free()

func set_task(_task: Task) -> void:
	assert(_task)
	task = _task
	
	if not _task.is_inside_tree():
		add_child(_task)
