class_name DismantleActorAction extends ActorAction

var target: ItemOnGround
var constructable: ConstructableComponent

func _init(actor: Settler, params: Dictionary) -> void:
	super._init(actor, params)
	target = params.target
	constructable = params.target.component_container.get_by_id(Components.Id.Constructable)
	constructable.no_durability_left.connect(func() -> void: finished.emit(self))

func process_action(delta: float) -> void:
	if constructable:
		constructable.reduce_durability(actor.dismantling_speed * delta)
