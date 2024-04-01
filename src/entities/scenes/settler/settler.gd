class_name Settler extends Entity

@onready var animation_player_audio: AnimationPlayer = $AnimationPlayerAudio

@export var utility_agent: UtilityAiAgent
@export var task_handler: TaskHandler

const REACH_DISTANCE := 2.5
const AT_DISTANCE := 1.0

var walk_speed := 5.0
var build_speed := 0.3
var dismantling_speed := 3
var open_door_speed := 1.2

var velocity := Vector3(0, 0, 0)

var valid_position_timer := 0.0
var valid_position_interval := 1.0

# TODO: If we end up needing this elsewhere, stick it somewhere global
# This is copied from Beehive which doesn't expose a name for the enum
enum TaskResult {
	SUCCESS,
	FAILURE,
	RUNNING
}

func prep_inventory(inventory_component: InventoryComponent) -> void:
	inventory_component.item_added.connect(self._inventory_item_added)
	inventory_component.item_removed.connect(self._inventory_item_removed)
	_refresh_carry_item()

func _ready() -> void:
	super._ready()
	name = "Settler"
	#Events.debug_visuals_set.connect(func(new_value: bool) -> void: $Line2D.visible = new_value)
	
	var original_position := global_position
	
	component_container.component_added.connect(func(component: Component) -> void:
		if component is InventoryComponent:
			prep_inventory(component)
		
		if component is DisplayNameComponent:
			component.display_name = ["Fred", "Mary", "Bob", "Susanne"].pick_random()
		
		if component is WorldPositionComponent:
			WorldPositionComponent.set_world_position(self, original_position)
		)
		
		#component_container.component_removed.connect(func(component_id: Components.Id) -> void:
			#if component_id == Components.Id.Inventory:
				#
			#)
				
	play_animation("Idle")

	utility_agent.top_score_action_changed.connect(_utility_ai_action_changed)

func _handle_new_component_container(old_container: ComponentContainer, new_container: ComponentContainer) -> void:
	pass

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
	velocity = Vector3.ZERO
	
	process_actions(delta)
	
	move_and_slide(delta)
	
	# TODO: Handle this betterer at some point
	valid_position_timer += delta
	if valid_position_timer >= valid_position_interval:
		ensure_valid_position()
		valid_position_timer = 0


func ensure_valid_position() -> void:
	if not is_in_valid_position():
		var free_coordinate := PathFinder.get_closest_free_point(Globals.get_map().global_position_to_coordinate(global_position)) as Vector2i
		if free_coordinate:
			var new_position := Globals.get_map().coordinate_to_global_position(free_coordinate)
			var position_component: WorldPositionComponent = component_container.get_by_id(Components.Id.WorldPosition)
			get_tree().create_tween().tween_property(position_component, "current_position", Vector3(new_position.x, 0.5, new_position.z), 0.5)

func is_in_valid_position() -> bool:
	return not Globals.get_map().is_coordinate_occupied(component_container.get_by_id(Components.Id.WorldPosition).coordinate)

func is_at_target(_target: Vector3) -> bool:
	return global_position.distance_to(_target) <= AT_DISTANCE

func can_reach_target(_target: Vector3) -> bool:
	return global_position.distance_to(_target) <= REACH_DISTANCE

func get_action_range() -> float:
	return REACH_DISTANCE

var actions: Array[ActorAction]

func has_action(action: ActorAction) -> bool:
	return actions.has(action)

func add_action(action: ActorAction) -> void:
	actions.append(action)
	action.finished.connect(func() -> void: actions.erase(action))

func process_actions(delta: float) -> void:
	for action in actions:
		if action.is_started():
			action.process_action(delta)

func _refresh_carry_item() -> void:
	var inventory: InventoryComponent = component_container.get_by_id(Components.Id.Inventory)
	var items := inventory.get_items()
	for child in $CarryItemNode.get_children():
		child.queue_free()
		
	if items.size() > 0:
		var first_item_amount: ItemAmountComponent = items[0]
		var item := first_item_amount.item
		var item_render_scene := Items.get_item_render_scene(item)
		if item_render_scene:
			$CarryItemNode.add_child(item_render_scene)
		

func _inventory_item_added(item: EntityDefinition, _amount: int) -> void:
	_refresh_carry_item()

func _inventory_item_removed(_item: EntityDefinition, _amount: int) -> void:
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

func play_eating_sound() -> void:
	var player: AudioStreamPlayer3D = $EatingSound
	player.pitch_scale = 1 + randf_range(-0.1, 0.1)
	player.play()

func _utility_ai_action_changed(utility_ai_task_id: String) -> void:
	task_handler.handle_utility_ai_task(utility_ai_task_id)
	#print("New action: %s" % utility_ai_task_id)

func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["task_handler"] = task_handler.serialize()
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	task_handler.deserialize(dict["task_handler"])
