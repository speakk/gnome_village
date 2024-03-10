class_name Task extends Node

var task_id: Tasks.TaskId
var task_name: String

signal failed(task: Task)

var is_being_worked_on := false

var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
		is_finished = new_value

var has_failed := false:
	set(new_value):
		has_failed = new_value
		if new_value:
			failed.emit(self)
			is_being_worked_on = false

func initialize(params: Dictionary) -> void:
	push_error("Task - Abstract initialize called. Did you forget to implement initialize?")

func save() -> Dictionary:
	var save_dict := {
		"is_being_worked_on" = is_being_worked_on,
		"is_finished" = is_finished,
		"has_failed" = has_failed,
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	is_being_worked_on = save_dict["is_being_worked_on"]
	is_finished = save_dict["is_finished"]
	has_failed = save_dict["has_failed"]
