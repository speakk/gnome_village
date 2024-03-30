class_name Task extends Node

var task_id: Tasks.TaskId
var task_name: String

var task_actuator_scene: PackedScene
var animation_name: String

signal failed()
signal cancelled()
signal finished()

var is_being_worked_on := false
var tree_root := false

enum OrderType {
	Sequence, Parallel, None
}

var order_type: OrderType = OrderType.None
var _subtasks: Array[Task]

var _parent_task: Task

var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
			finished.emit()
			print("Task: %s finished" % task_name)
			clean_up(false)
		is_finished = new_value

var has_failed := false:
	set(new_value):
		has_failed = new_value
		if new_value:
			failed.emit()
			print("Task: %s failed" % task_name)
			is_being_worked_on = false

var is_cancelled := false:
	set(new_value):
		is_cancelled = new_value
		if new_value:
			clean_up(true)

func register_subtask(task: Task) -> void:
	_subtasks.append(task)
	task._parent_task = self
	task.finished.connect(func() -> void:
		_subtasks.erase(task)
		if _subtasks.size() <= 0:
			is_finished = true
		)
	
	task.failed.connect(func() -> void:
		failed.emit()
		)

func get_subtasks() -> Array[Task]:
	return _subtasks

func is_root() -> bool:
	return _parent_task is Task

func is_leaf() -> bool:
	return _subtasks.size() == 0

func clean_up(_cancelled: bool) -> void:
	if _cancelled:
		cancelled.emit()
		
	for subtask in _subtasks:
		subtask.clean_up(_cancelled)
	
	queue_free()

func create_action(actor: Settler) -> ActorAction:
	push_error("Abstract task create_action called")
	return null

func get_target(actor: Settler) -> Vector3:
	return Vector3(-9999, -9999, -9999)

func serialize() -> Dictionary:
	var dict: Dictionary = {
		resource_path = scene_file_path,
		task_id = task_id,
		task_name = task_name,
		animation_name = animation_name,
		task_actuator_scene_path = task_actuator_scene.resource_path,
		is_being_worked_on = is_being_worked_on,
		is_finished = is_finished,
		has_failed = has_failed,
	}
	
	if _parent_task:
		dict["_parent_task_id"] = SaveSystem.get_save_id(_parent_task)

	dict["_subtasks"] = _subtasks.map(func(subtask: Task) -> Dictionary:
		return subtask.serialize()
		)
	
	return dict

func deserialize(dict: Dictionary) -> void:
	task_id = dict["task_id"]
	task_name = dict["task_name"]
	animation_name = dict["animation_name"]
	task_actuator_scene = load(dict["task_actuator_scene_path"])
	is_being_worked_on = dict["is_being_worked_on"]
	is_finished = dict["is_finished"]
	has_failed = dict["has_failed"]
	_subtasks.assign(dict["_subtasks"].map(func(subtask_dict: Dictionary) -> Task:
		var subtask := static_deserialize(subtask_dict)
		subtask.deserialize(subtask_dict)
		return subtask
		))

static func static_deserialize(dict: Dictionary) -> Task:
	var task: Task = load(dict["resource_path"]).initialize()
	return task
