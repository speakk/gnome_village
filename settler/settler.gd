extends CharacterBody2D

class_name Settler

const TARGET_DISTANCE_TRESHOLD := 10.0

var walk_speed := 100.0
var build_speed := 0.3

var current_task: Task

var target: Variant # Vector2 | Null
var build_target: Blueprint

var path: Variant # PackedVector2Array | Null
var current_path_index: int = 0

# TODO: If we end up needing this elsewhere, stick it somewhere global
# This is copied from Beehive which doesn't expose a name for the enum
enum TaskResult {
	SUCCESS,
	FAILURE,
	RUNNING
}

func _ready() -> void:
	Events.task_finished.connect(_task_finished)
	#process_mode = Node.PROCESS_MODE_DISABLED

func _task_finished(task: Task) -> void:
	if current_task == task:
		current_task = null

#func _process(delta: float) -> void:
	#if current_task:
		##print("Ticking?")
		#current_task.tick()

func get_direction_to_next_path_point() -> Vector2:
	#print("Next path point: ", PathFinder.get_point_position(path[current_path_index]))
	#var point_position := PathFinder.get_point_position(path[current_path_index])
	var point_position := path[current_path_index] as Vector2
	return global_position.direction_to(Globals.get_map().map_to_local(point_position))

func advance_path_index() -> void:
	if path:
		if global_position.distance_to(Globals.get_map().map_to_local(path[current_path_index])) < TARGET_DISTANCE_TRESHOLD / 3:
			current_path_index += 1
			if current_path_index > path.size() - 1:
				# TODO: Emit path finished event if needed?
				#_finished_path()
				path = null
				current_path_index = 0

func _finished_path() -> void:
	pass

func _physics_process(delta: float) -> void:
	velocity = Vector2.ZERO
	advance_path_index()
	
	#if target:
		#velocity = global_position.direction_to(target) * walk_speed
	
	if path:
		velocity = get_direction_to_next_path_point() * walk_speed
	
	if build_target:
		build_target.increase_build_progress(build_speed * delta)
	
	if velocity.length() > 0.1:
		$AnimationPlayer.play("walk")
		
		if velocity.x > 0:
			$Sprite.flip_h = false
		else:
			$Sprite.flip_h = true
		
	else:
		$AnimationPlayer.play("idle")
	
	move_and_slide()

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
	if !target or (_target and not (_target as Vector2).is_equal_approx(target)):
		var map_position_from := Globals.get_map().local_to_map(global_position)
		var map_position_to := Globals.get_map().local_to_map(_target)
		path = PathFinder.get_id_path(map_position_from, map_position_to)
		current_path_index = 0
	target = _target
	
	
func set_build_target(_build_target: Variant) -> void:
	build_target = _build_target

func is_next_to_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= TARGET_DISTANCE_TRESHOLD * 2
