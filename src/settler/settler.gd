extends Node3D

class_name Settler

@onready var animation_player_audio: AnimationPlayer = $AnimationPlayerAudio
@onready var component_container: ComponentContainer = $ComponentContainer
@onready var inventory: InventoryComponent = component_container.get_by_id(Components.Id.Inventory)


const REACH_DISTANCE := 2.5
const AT_DISTANCE := 1.0

var walk_speed := 5.0
var build_speed := 0.3
var dismantling_speed := 3
var open_door_speed := 1.2

var velocity := Vector3(0, 0, 0)

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
	inventory.item_added.connect(_inventory_item_added)
	inventory.item_removed.connect(_inventory_item_removed)
	play_animation("Idle")
	
	var original_position := global_position
	
	component_container.get_by_id(Components.Id.DisplayName).display_name = ["Fred", "Mary", "Bob", "Susanne"].pick_random()
	WorldPositionComponent.set_world_position(self, original_position)


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
	
	save_dict["inventory_id"] = SaveSystem.save_entity(inventory)
	
	if current_task_actuator:
		#save_dict["current_task_actuator_id"] = SaveSystem.save_entity(current_task_actuator)
		save_dict["task_id"] = SaveSystem.save_entity(current_task_actuator.task)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	var position_component: WorldPositionComponent = component_container.get_by_id(Components.Id.WorldPosition)
	position_component.current_position = Vector3(save_dict["position_x"], 0.5, save_dict["position_y"])
	walk_speed = save_dict["walk_speed"]
	build_speed = save_dict["build_speed"]
	dismantling_speed = save_dict["dismantling_speed"]
	open_door_speed = save_dict["open_door_speed"]
	velocity.x = save_dict["velocity_x"]
	velocity.y = save_dict["velocity_y"]
	#if save_dict.has("current_task_actuator_id"):
		#current_task_actuator = SaveSystem.get_saved_entity(save_dict["current_task_actuator_id"])
		#add_child(current_task_actuator)
	
	if save_dict.has("task_id"):
		var task := SaveSystem.get_saved_entity(save_dict["task_id"]) as Task
		#add_child(current_task_actuator)
		start_task(task)
	
	inventory = SaveSystem.get_saved_entity(save_dict["inventory_id"])

	_refresh_carry_item()
	
	#if save_dict.has("current_task_save_id"):
	#	SaveSystem.register_load_reference(self, "current_task_actuator", save_dict["current_task_save_id"], true)

func _finished_path() -> void:
	pass

func move_and_slide(delta: float) -> void:
	var position_component: WorldPositionComponent = component_container.get_by_id(Components.Id.WorldPosition)
	position_component.current_position += velocity * delta
	position_component.current_position.y = 0.5
	#global_position += velocity * delta
	# 3D rework: Fix this elsewhere
	global_position.y = 0.5
#
#func _process(delta: float) -> void:
	##$Line2D.global_position = get_parent().global_position

func _physics_process(delta: float) -> void:
	if not current_task_actuator:
		current_task_actuator = null
		#clear_path()
	
	velocity = Vector3.ZERO
	
	process_actions(delta)
	
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
	if current_task_actuator:
		current_task_actuator.task.failed.disconnect(_task_failed)
		current_task_actuator.task.tree_exited.disconnect(_clean_up_actuator)
		remove_child(current_task_actuator)
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

func ensure_valid_position() -> void:
	if not is_in_valid_position():
		var free_coordinate := PathFinder.get_closest_free_point(Globals.get_map().global_position_to_coordinate(global_position)) as Vector2i
		if free_coordinate:
			var new_position := Globals.get_map().coordinate_to_global_position(free_coordinate)
			var position_component: WorldPositionComponent = component_container.get_by_id(Components.Id.WorldPosition)
			get_tree().create_tween().tween_property(position_component, "current_position", Vector3(new_position.x, 0.5, new_position.z), 0.5)
			#global_position = new_position

func is_in_valid_position() -> bool:
	#var entities := Globals.get_map().get_map_entities(Globals.get_map().global_position_to_coordinate(global_position))
	return not Globals.get_map().is_coordinate_occupied(component_container.get_by_id(Components.Id.WorldPosition).coordinate)
	#return not PathFinder.is_position_solid(component_container.get_by_id(Components.Id.WorldPosition).coordinate)

func is_at_target(_target: Vector3) -> bool:
	return global_position.distance_to(_target) <= AT_DISTANCE

func can_reach_target(_target: Vector3) -> bool:
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

func _refresh_carry_item() -> void:
	var items := inventory.get_items()
	for child in $CarryItemNode.get_children():
		child.queue_free()
		
	if items.size() > 0:
		var first_item_amount: ItemAmountComponent = items[0]
		var item := Items.get_by_id(first_item_amount.item_id)
		var item_render_scene := Items.get_item_render_scene(item)
		if item_render_scene:
			$CarryItemNode.add_child(item_render_scene)
		

func _inventory_item_added(item_id: Variant, _amount: int) -> void:
	_refresh_carry_item()

func _inventory_item_removed(_item_id: Variant, _amount: int) -> void:
	_refresh_carry_item()

func play_animation(animation_name: String) -> void:
	if not $settler/AnimationPlayer.current_animation == animation_name:
		$settler/AnimationPlayer.play(animation_name)

		# Test code for continous Build sound
		#await get_tree().create_timer(randf_range(0.5, 2)).timeout
		#$AnimationPlayerAudio.play("Build")
		
		if $AnimationPlayerAudio.has_animation(animation_name):
			$AnimationPlayerAudio.play(animation_name)
		else:
			$AnimationPlayerAudio.stop()
	
func stop_animation() -> void:
	$settler/AnimationPlayer.stop()
	$AnimationPlayerAudio.stop()

func play_hammer_sound() -> void:
	var player := $HammerSounds.get_children().pick_random() as AudioStreamPlayer3D
	player.play()
	
