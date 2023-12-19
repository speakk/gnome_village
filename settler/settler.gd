extends CharacterBody2D

class_name Settler

const TARGET_DISTANCE_TRESHOLD := 10.0

var walk_speed := 100.0
var build_speed := 0.3

var current_task: Task

var target: Variant # Vector2 | Null
var build_target: Blueprint

# TODO: If we end up needing this elsewhere, stick it somewhere global
# This is copied from Beehive which doesn't expose a name for the enum
enum TaskResult {
	SUCCESS,
	FAILURE,
	RUNNING
}

func _ready() -> void:
	Events.task_finished.connect(_task_finished)

func _task_finished(task: Task) -> void:
	if current_task == task:
		current_task = null

#func _process(delta: float) -> void:
	#if current_task:
		##print("Ticking?")
		#current_task.tick()

func _physics_process(delta: float) -> void:
	if target:
		global_position += global_position.direction_to(target) * delta * walk_speed
	
	if build_target:
		build_target.increase_build_progress(build_speed * delta)

func get_current_task() -> Task:
	return current_task

func start_task(task: Task) -> void:
	current_task = task
	current_task.is_being_worked_on = true
	current_task.enable()
	current_task.actor = self
	add_child(current_task)

func finish_current_task() -> void:
	current_task.is_finished = true
	current_task.is_being_worked_on = false
	remove_child(current_task)
	current_task = null
	Events.task_finished.emit(current_task)
	# TODO: Queue free task at some point maybe... Not now though

#func tick_current_task() -> int:
	#return current_task.tick()

func is_available_for_work() -> bool:
	return current_task == null

func get_task_status() -> int:
	return current_task.get_last_tick_status()

func set_target(_target: Variant) -> void:
	target = _target
	
func set_build_target(_build_target: Variant) -> void:
	build_target = _build_target

func is_next_to_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= TARGET_DISTANCE_TRESHOLD
