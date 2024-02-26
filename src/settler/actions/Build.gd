class_name BuildActorAction extends ActorAction

var constructable_component: ConstructableComponent

func initialize(params: Variant) -> ActorAction:
	constructable_component = params.constructable_component
	constructable_component.finished.connect(func() -> void: finished.emit(self))
	return self

func process_action(actor: Settler, delta: float) -> void:
	#print("Processing build action")
	constructable_component.increase_progress(actor.build_speed * delta)
	
	var target_position: Vector3 = constructable_component.get_owner().global_position
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target != actor.global_position:
		actor.look_at(look_at_target, Vector3.UP, true)
