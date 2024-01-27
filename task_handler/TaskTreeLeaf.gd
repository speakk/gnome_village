extends Node

class_name TaskTreeLeaf

var task_id: Tasks.TaskId

#func clean_up() -> void:
	##task.queue_free()

func set_task_id(_task_id: Tasks.TaskId) -> void:
	assert(_task_id)
	task_id = _task_id
