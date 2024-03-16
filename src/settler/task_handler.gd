class_name TaskHandler extends Node

var current_task_actuator: TaskActuator

@export var settler: Settler

var latest_task_id: String

# TODO: Utility AI changing state is already a signal
# We only really need to re-check if task starting failed.
# Anyway, that is for a future optimization
var _process_interval: float = randf_range(1, 4)
var _current_process_timer := _process_interval

func handle_utility_ai_task(task_id: String) -> void:
	latest_task_id = task_id

func _physics_process(delta: float) -> void:
	_current_process_timer -= delta
	if _current_process_timer > 0:
		return
	
	_current_process_timer = _process_interval
	
	if not self.current_task_actuator:
		match latest_task_id:
			"eat":
				var consumables := get_tree().get_nodes_in_group("consumable")
				if consumables.size() > 0:
					for consumable in consumables:
						var component: ConsumableComponent = consumable.component_container.get_by_id(Components.Id.Consumable)
						if not component.reserved:
							var eating_task := EatTask.new({ consumable = component })
							start_task(eating_task)
							break
			"sleep":
				var task := SleepTask.new()
				start_task(task)
			"work":
				var next_work_task: Task = TaskManager.get_available_task(settler.global_position)
				if next_work_task:
					start_task(next_work_task)

func get_current_task() -> TaskActuator:
	return current_task_actuator

func start_task(task: Task) -> void:
	var task_actuator: TaskActuator
	if task.task_actuator_scene:
		task_actuator = task.task_actuator_scene.instantiate()
	else:
		task_actuator = preload("res://src/tasks/task_actuators/simple_actuator.tscn").instantiate()
		
	task_actuator.initialize(task)
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

func _task_failed() -> void:
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
