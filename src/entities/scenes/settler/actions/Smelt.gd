class_name SmeltActorAction extends ActorTaskAction

var smelter_component: SmelterComponent

func validate(_actor: Settler, task: Task) -> void:
	task = task as BuildTask
	if not actor.can_reach_target(task.constructable_component.get_owner().global_position):
		validation_failed.emit()

func _init(_actor: Settler, task: Task) -> void:
	super._init(_actor, task)
	task = task as BuildTask
	smelter_component = task.smelter_component
	smelter_component.smelt_finished.connect(func() -> void: finished.emit())

func process_action(delta: float) -> void:
	smelter_component.increase_progress(actor.smelt_speed * delta)
	
	var target_position: Vector3 = smelter_component.get_owner().global_position
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target.distance_squared_to(actor.global_position) > 0.01:
		actor.look_at(look_at_target, Vector3.UP, true)

