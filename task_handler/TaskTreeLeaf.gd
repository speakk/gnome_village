extends Node

class_name TaskTreeLeaf

var task: Task

func clean_up() -> void:
	task.queue_free()
