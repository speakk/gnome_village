class_name BuildActorAction extends ActorAction

var constructable_component: ConstructableComponent

func validate(actor: Settler, params: Dictionary) -> void:
	if not actor.can_reach_target(params.constructable_component.get_owner().global_position):
		validation_failed.emit()

func _init(actor: Settler, params: Dictionary) -> void:
	super._init(actor, params)
	constructable_component = params.constructable_component
	constructable_component.finished.connect(func() -> void: finished.emit(self))

func process_action(delta: float) -> void:
	constructable_component.increase_progress(actor.build_speed * delta)
	
	var target_position: Vector3 = constructable_component.get_owner().global_position
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target.distance_squared_to(actor.global_position) > 0.01:
		actor.look_at(look_at_target, Vector3.UP, true)
