class_name DismantleActorAction extends ActorAction

var target: ItemOnGround

func initialize(params: Variant) -> ActorAction:
	target = params.target
	return self

func process_action(actor: Settler, delta: float) -> void:
	target.reduce_durability(actor.dismantling_speed * delta)
	if not target.has_durability_left():
		finished.emit(self)
