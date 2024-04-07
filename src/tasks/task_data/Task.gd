class_name Task extends Node

var task_id: Tasks.TaskId
var task_name: String

var task_actuator_scene: PackedScene
var animation_name: String
var timeout_length: float = 5 # Seconds
@onready var _timeout_timer: float = timeout_length # Seconds

signal failed()
signal cancelled()
signal finished()
signal removed()

var is_being_worked_on := false
var is_on_timeout := false


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
			is_on_timeout = true

var is_cancelled := false:
	set(new_value):
		is_cancelled = new_value
		if _parent_task:
			cancelled.emit()
		else:
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
	
	task.cancelled.connect(func() -> void:
		is_cancelled = true
		)
	
func get_subtasks() -> Array[Task]:
	return _subtasks

func is_root() -> bool:
	return _parent_task is Task

func is_leaf() -> bool:
	return _subtasks.size() == 0

func clean_up(_cancelled: bool, should_emit_signal: bool = true) -> void:
	if _cancelled:
		if should_emit_signal:
			cancelled.emit()
		
	for subtask in _subtasks:
		subtask.clean_up(_cancelled, false)
	
	removed.emit()
	queue_free()

func create_action(actor: Settler) -> ActorAction:
	push_error("Abstract task create_action called")
	return null

func get_target(actor: Settler) -> Vector3:
	return Vector3(-9999, -9999, -9999)

func serialize() -> Dictionary:
	var dict: Dictionary = {
		"resource_path": get_script().get_path(),
		"task_id": task_id,
		"task_name": task_name,
		"order_type": order_type,
		"animation_name": animation_name,
		"is_being_worked_on": is_being_worked_on,
		"is_finished": is_finished,
		"has_failed": has_failed,
		"save_id": SaveSystem.get_save_id(self)
	}
	
	if task_actuator_scene:
		dict["task_actuator_scene_path"] = task_actuator_scene.resource_path
	
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
	order_type = dict["order_type"]
	is_being_worked_on = dict["is_being_worked_on"]
	is_finished = dict["is_finished"]
	has_failed = dict["has_failed"]
	
	set_meta("save_id", dict["save_id"])
	SaveSystem.register_entity_reference(self)
	
	if dict.has("task_actuator_scene_path"):
		task_actuator_scene = load(dict["task_actuator_scene_path"])
	
	for subtask_dict: Dictionary in dict["_subtasks"]:
		var subtask := static_deserialize(subtask_dict)
		subtask.deserialize(subtask_dict)
		register_subtask(subtask)

	
	if dict.has("_parent_task_id"):
		SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["_parent_task_id"], func(new_parent: Variant) -> void:
			_parent_task = new_parent
			))

static func static_deserialize(dict: Dictionary) -> Task:
	var task: Task = load(dict["resource_path"]).new()
	return task

func _process(delta: float) -> void:
	for subtask in _subtasks:
		subtask._process(delta)
	
	if is_on_timeout:
		_timeout_timer -= delta
		if _timeout_timer <= 0:
			_timeout_timer = timeout_length
			is_on_timeout = false
