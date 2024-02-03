extends Node2D

class_name Settler

@onready var persistent: Persistent = get_node("Persistent") as Persistent

#@onready var hair_options: Array[Texture2D] = [
	#preload("res://assets/settler_hair_1.png"), preload("res://assets/settler_hair_2.png"), null
#]

const REACH_DISTANCE := MainMap.CELL_SIZE.x * 1.5
const AT_DISTANCE := 10.0

var walk_speed := 100.0
var build_speed := 0.3
var dismantling_speed := 3
var open_door_speed := 0.6

var velocity := Vector2(0, 0)

var current_task_actuator: TaskActuator

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
	$Inventory.item_added.connect(_inventory_item_added)
	$Inventory.item_removed.connect(_inventory_item_removed)
	#
	#var hair := hair_options.pick_random() as Texture2D
	#if hair:
		#$HairSprite.texture = hair
	#else:
		#$HairSprite.visible = false

func save() -> Dictionary:
	var save_dict := {
		"position_x" = global_position.x,
		"position_y" = global_position.y,
		"walk_speed" = walk_speed,
		"build_speed" = build_speed,
		"dismantling_speed" = dismantling_speed,
		"open_door_speed" = open_door_speed,
		"velocity_x" = velocity.x,
		"velocity_y" = velocity.y,
	}
	
	save_dict["inventory_id"] = SaveSystem.save_entity($Inventory)
	
	if current_task_actuator:
		save_dict["current_task_save_id"] = SaveSystem.save_entity(current_task_actuator)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	global_position.x = save_dict["position_x"]
	global_position.y = save_dict["position_y"]
	walk_speed = save_dict["walk_speed"]
	build_speed = save_dict["build_speed"]
	dismantling_speed = save_dict["dismantling_speed"]
	open_door_speed = save_dict["open_door_speed"]
	velocity.x = save_dict["velocity_x"]
	velocity.y = save_dict["velocity_y"]
	if save_dict.has("current_task_save_id"):
		current_task_actuator = SaveSystem.get_saved_entity(save_dict["current_task_save_id"])
		add_child(current_task_actuator)
	
	var inventory: Variant = SaveSystem.get_saved_entity(save_dict["inventory_id"])
	$Inventory.queue_free()
	$Inventory.name = "old_inventory"
	add_child(inventory)
	inventory.name = "Inventory"
	
	#if save_dict.has("current_task_save_id"):
	#	SaveSystem.register_load_reference(self, "current_task_actuator", save_dict["current_task_save_id"], true)

func _finished_path() -> void:
	pass

func move_and_slide(delta: float) -> void:
	global_position += velocity * delta

func _process(delta: float) -> void:
	$Line2D.global_position = get_parent().global_position

func _physics_process(delta: float) -> void:
	if not current_task_actuator:
		current_task_actuator = null
		#clear_path()
	
	velocity = Vector2.ZERO
	
	process_actions(delta)
	
	if velocity.length() > 0:
		$AnimationPlayer.play("walk_still")
		
		if velocity.x > 0:
			$Sprite.flip_h = false
			#$HairSprite.flip_h = false
		else:
			$Sprite.flip_h = true
			#$HairSprite.flip_h = true
		
	else:
		$AnimationPlayer.play("idle")
	
	move_and_slide(delta)
	
	# TODO: Handle this betterer at some point
	valid_position_timer += delta
	if valid_position_timer >= valid_position_interval:
		ensure_valid_position()
		valid_position_timer = 0

func get_current_task() -> TaskActuator:
	return current_task_actuator

func start_task(task: Task) -> void:
	var task_actuator := Tasks.create_task_actuator(task)
	task.tree_exited.connect(_clean_up_actuator)
	task.failed.connect(_task_failed)
	add_child(task_actuator)
	current_task_actuator = task_actuator
	current_task_actuator.start_work()

func _clean_up_actuator() -> void:
	current_task_actuator.task.failed.disconnect(_task_failed)
	current_task_actuator.task.tree_exited.disconnect(_clean_up_actuator)
	remove_child(current_task_actuator)
	current_task_actuator = null

func _task_failed(_task: Task) -> void:
	_clean_up_actuator()

func finish_current_task() -> void:
	current_task_actuator.finish()
	remove_child(current_task_actuator)
	current_task_actuator = null
	# TODO: Queue free task at some point maybe... Not now though

func is_available_for_work() -> bool:
	return current_task_actuator == null or not current_task_actuator

func get_task_status() -> int:
	return current_task_actuator.get_last_tick_status()

func ensure_valid_position() -> void:
	if not is_in_valid_position():
		var free_coordinate := PathFinder.get_closest_free_point(Globals.get_map().global_position_to_coordinate(global_position)) as Vector2i
		if free_coordinate:
			var new_position := Globals.get_map().coordinate_to_global_position(free_coordinate)
			get_tree().create_tween().tween_property(self, "global_position", new_position, 0.5)
			#global_position = new_position

func is_in_valid_position() -> bool:
	var entities := Globals.get_map().get_map_entities(Globals.get_map().global_position_to_coordinate(global_position))
	for entity in entities:
		if entity.item.is_solid:
			return false
	
	return true
	#return not PathFinder.is_position_solid(Globals.get_map().global_position_to_coordinate(global_position))

func is_at_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= AT_DISTANCE

func can_reach_target(_target: Vector2) -> bool:
	return global_position.distance_to(_target) <= REACH_DISTANCE

func get_action_range() -> float:
	return REACH_DISTANCE

var actions: Array[ActorAction]

func add_action(action: ActorAction) -> void:
	actions.append(action)
	action.finished.connect(func(_action: ActorAction) -> void: actions.erase(_action))

func process_actions(delta: float) -> void:
	for action in actions:
		action.process_action(self, delta)
	
func _inventory_item_added(item_id: Variant, _amount: int) -> void:
	var item := Items.get_by_id(item_id)
	Items.copy_item_properties_to_sprite(item, $CarryItemSprite)
	$CarryItemSprite.show()

func _inventory_item_removed(_item_id: Variant, _amount: int) -> void:
	$CarryItemSprite.hide()
