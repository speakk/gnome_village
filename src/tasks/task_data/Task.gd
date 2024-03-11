class_name Task extends Node

var task_id: Tasks.TaskId
var task_name: String

signal failed()
signal finished()

var is_being_worked_on := false
var tree_root := false

enum OrderType {
	Sequence, Parallel, None
}

var order_type: OrderType = OrderType.None
var _subtasks: Array[Task]

var _parent_task: Task

func register_subtask(task: Task) -> void:
	_subtasks.append(task)
	task._parent_task = self
	task.finished.connect(func() -> void:
		_subtasks.erase(task)
		if _subtasks.size() <= 0:
			finished.emit()
			if tree_root:
				clean_up()
		)
	
	task.failed.connect(func() -> void:
		print("Handle subtask failure TODO")
		)

func get_subtasks() -> Array[Task]:
	return _subtasks

func is_root() -> bool:
	return _parent_task is Task

func is_leaf() -> bool:
	return _subtasks.size() == 0

func clean_up() -> void:
	for subtask in _subtasks:
		subtask.clean_up()
	
	queue_free()


var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
			finished.emit()
		is_finished = new_value

var has_failed := false:
	set(new_value):
		has_failed = new_value
		if new_value:
			failed.emit(self)
			is_being_worked_on = false

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
