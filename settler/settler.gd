extends CharacterBody2D

class_name Settler

var walk_speed := 100.0

var current_task: Task

var target: Vector2

# TODO: If we end up needing this elsewhere, stick it somewhere global
# This is copied from Beehive which doesn't expose a name for the enum
enum TaskResult {
	SUCCESS,
	FAILURE,
	RUNNING
}

func _physics_process(delta: float) -> void:
	if target:
		global_position += global_position.direction_to(target) * delta * walk_speed

func get_current_task() -> Task:
	return current_task

func start_task(task: Task) -> void:
	if current_task:
		# TODO: Do something with previous task?
		pass
	
	current_task = task
	await get_tree().process_frame
	#current_task.actor = self
	print("Path", get_path())
	print("Path self", get_path_to(self))
	current_task.actor_node_path = get_path()
	
#func tick_current_task() -> int:
	#return current_task.tick()

func get_task_status() -> int:
	print("Status ", current_task.get_last_tick_status())
	return current_task.get_last_tick_status()

func set_target(_target: Vector2) -> void:
	target = _target
