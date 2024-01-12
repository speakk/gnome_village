extends Node2D

class_name Settler

const REACH_DISTANCE := MainMap.CELL_SIZE.x * 1.5
const AT_DISTANCE := 10.0

var walk_speed := 100.0
var build_speed := 0.3
var dismantling_speed := 3

var velocity := Vector2(0, 0)

var current_task: Task

var target: Variant # Vector2 | Null
var task_target: Variant


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
	Events.debug_visuals_set.connect(func(new_value: bool) -> void: $Line2D.visible = new_value)

func _finished_path() -> void:
	pass

func move_and_slide(delta: float) -> void:
	global_position += velocity * delta

func _process(delta: float) -> void:
	$Line2D.global_position = get_parent().global_position

func _physics_process(delta: float) -> void:
	if not current_task:
		current_task = null
		#clear_path()
	
	velocity = Vector2.ZERO
	
	if velocity.length() > 0.1:
		$AnimationPlayer.play("walk")
		
		if velocity.x > 0:
			$Sprite.flip_h = false
		else:
			$Sprite.flip_h = true
		
	else:
		$AnimationPlayer.play("idle")
	
	process_actions(delta)
	
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

func ensure_valid_position() -> void:
	if not is_in_valid_position():
		var free_coordinate := PathFinder.get_closest_free_point(Globals.get_map().global_position_to_coordinate(global_position)) as Vector2i
		if free_coordinate:
			var new_position := Globals.get_map().coordinate_to_global_position(free_coordinate)
			global_position = new_position

func is_in_valid_position() -> bool:
	return not PathFinder.is_position_solid(Globals.get_map().global_position_to_coordinate(global_position))

func is_at_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= AT_DISTANCE

func can_reach_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= REACH_DISTANCE

var actions: Array[ActorAction]

func add_action(action: ActorAction) -> void:
	actions.append(action)
	action.finished.connect(func(_action: ActorAction) -> void: actions.erase(_action))

func process_actions(delta: float) -> void:
	for action in actions:
		action.process_action(self, delta)
