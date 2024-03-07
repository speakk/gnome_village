class_name DismantleActorAction extends ActorAction

var target: ItemOnGround
var constructable: ConstructableComponent

func initialize(params: Variant) -> ActorAction:
	target = params.target
	constructable = params.target.component_container.get_by_id(Components.Id.Constructable)
	constructable.no_durability_left.connect(func() -> void: finished.emit(self))
	return self

func process_action(actor: Settler, delta: float) -> void:
	if constructable:
		constructable.reduce_durability(actor.dismantling_speed * delta)
