class_name TaskHandler extends Node

var current_task_actuator: TaskActuator

@export var settler: Settler

var latest_task_id: String

func handle_utility_ai_task(task_id: String) -> void:
	latest_task_id = task_id

func _physics_process(delta: float) -> void:
	if not current_task_actuator:
		match latest_task_id:
			"eat":
				# TODO: start_task for eat task here,
				print("Eat!")
			"sleep":
				# TODO: start_task for sleep task here,
				print("Sleep!")
			"work":
				var next_work_task: Task = TaskManager.get_available_task(settler.global_position)
				if next_work_task:
					start_task(next_work_task)

func get_current_task() -> TaskActuator:
	return current_task_actuator

func start_task(task: Task) -> void:
	var task_actuator := Tasks.create_task_actuator(task)
	task.tree_exited.connect(_clean_up_actuator)
	task.failed.connect(_task_failed)
	task.cancelled.connect(func() -> void: _clean_up_actuator())
	settler.add_child(task_actuator)
	current_task_actuator = task_actuator
	current_task_actuator.start_work()
	

func _clean_up_actuator() -> void:
	if current_task_actuator:
		current_task_actuator.task.failed.disconnect(_task_failed)
		current_task_actuator.task.tree_exited.disconnect(_clean_up_actuator)
		settler.remove_child(current_task_actuator)
		settler.stop_animation()
		current_task_actuator = null

func _task_failed(_task: Task) -> void:
	_clean_up_actuator()

func finish_current_task() -> void:
	current_task_actuator.finish()
	_clean_up_actuator()

func fail_current_task() -> void:
	current_task_actuator.fail()
	_clean_up_actuator()

func is_available_for_work() -> bool:
	return current_task_actuator == null or not current_task_actuator

func get_task_status() -> int:
	return current_task_actuator.get_last_tick_status()