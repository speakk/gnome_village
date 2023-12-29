extends Node2D

class_name Settler

const REACH_DISTANCE := MainMap.CELL_SIZE.x * 1.5
const AT_DISTANCE := 10.0

var walk_speed := 100.0
var build_speed := 0.3

var velocity := Vector2(0, 0)

var current_task: Task

var target: Variant # Vector2 | Null
var build_target: Blueprint

var path: Variant: # PackedVector2Array | Null
	set(new_path):
		path = new_path
		if new_path:
			$Line2D.points = Array(new_path).map(func(point: Vector2i) -> Vector2:
				return Globals.get_map().coordinate_to_global_position(point)
			)
		else:
			$Line2D.points = []
var current_path_index: int = 0

var valid_position_timer := 0.0
var valid_position_interval := 1.0

# TODO: If we end up needing this elsewhere, stick it somewhere global
# This is copied from Beehive which doesn't expose a name for the enum
enum TaskResult {
	SUCCESS,
	FAILURE,
	RUNNING
}

func _ready() -> void:
	name = "Settler"

func get_direction_to_next_path_point() -> Vector2:
	#print("Next path point: ", PathFinder.get_point_position(path[current_path_index]))
	#var point_position := PathFinder.get_point_position(path[current_path_index])
	var point_position := path[current_path_index] as Vector2
	return global_position.direction_to(Globals.get_map().coordinate_to_global_position(point_position))

func clear_path() -> void:
	path = null
	current_path_index = 0
	target = null

func advance_path_index() -> void:
	if path:
		var distance := global_position.distance_to(Globals.get_map().coordinate_to_global_position(path[current_path_index]))
		if distance < AT_DISTANCE or (current_path_index == path.size() - 2 and distance < REACH_DISTANCE):
			current_path_index += 1
			if current_path_index > path.size() - 1:
				# TODO: Emit path finished event if needed?
				#_finished_path()
				clear_path()

func _finished_path() -> void:
	pass

func move_and_slide(delta: float) -> void:
	global_position += velocity * delta

func _process(delta: float) -> void:
	$Line2D.global_position = get_parent().global_position

func _physics_process(delta: float) -> void:
	if not current_task:
		current_task = null
		clear_path()
	
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
	
	move_and_slide(delta)
	
	# TODO: Handle this betterer at some point
	valid_position_timer += delta
	if valid_position_timer >= valid_position_interval:
		ensure_valid_position()
		valid_position_timer = 0

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
	# TODO: Queue free task at some point maybe... Not now though

func is_available_for_work() -> bool:
	return current_task == null or not current_task

func get_task_status() -> int:
	return current_task.get_last_tick_status()

func set_target(_target: Variant) -> void:
	if !target or (_target and not (_target as Vector2).is_equal_approx(target)):
		var map_position_from := Globals.get_map().global_position_to_coordinate(global_position)
		var map_position_to := Globals.get_map().global_position_to_coordinate(_target)
		path = PathFinder.get_id_path(map_position_from, map_position_to)
		current_path_index = 0
	target = _target

func ensure_valid_position() -> void:
	if not is_in_valid_position():
		var free_coordinate := PathFinder.get_closest_free_point(Globals.get_map().global_position_to_coordinate(global_position)) as Vector2i
		if free_coordinate:
			var new_position := Globals.get_map().coordinate_to_global_position(free_coordinate)
			global_position = new_position

func is_in_valid_position() -> bool:
	return not PathFinder.is_position_solid(Globals.get_map().global_position_to_coordinate(global_position))
	
func set_build_target(_build_target: Variant) -> void:
	build_target = _build_target

func is_at_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= AT_DISTANCE

func can_reach_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= REACH_DISTANCE
