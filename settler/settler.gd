extends CharacterBody2D

class_name Settler

var current_task: Task

func get_current_task() -> Task:
	return current_task

func start_task(task: Task) -> void:
	if current_task:
		# TODO: Do something with previous task?
		pass
	
	current_task = task

func tick_current_task() -> int:
	return current_task.tick()
