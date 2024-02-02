class_name DismantleActorAction extends ActorAction

var target: ItemOnGround

func initialize(params: Variant) -> ActorAction:
	target = params.target
	return self

func process_action(actor: Settler, delta: float) -> void:
	# TODO: Probably should not even end up in process_action if target has
	# already been freed
	if target:
		target.reduce_durability(actor.dismantling_speed * delta)
		if not target.has_durability_left():
			finished.emit(self)
