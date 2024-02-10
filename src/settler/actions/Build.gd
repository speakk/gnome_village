class_name BuildActorAction extends ActorAction

var build_target: ItemOnGround

func initialize(params: Variant) -> ActorAction:
	build_target = params.build_target
	return self

func process_action(actor: Settler, delta: float) -> void:
	#print("Processing build action")
	build_target.increase_build_progress(actor.build_speed * delta)
	if build_target.is_finished():
		finished.emit(self)
	
	var target_position := build_target.global_position
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target != actor.global_position:
		actor.look_at(look_at_target, Vector3.UP, true)
