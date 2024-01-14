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
