class_name DismantleActorAction extends ActorTaskAction

var target: Entity
var constructable: ConstructableComponent

func validate_task(actor: Settler, task: Task) -> void:
	task = task as DismantleTask
	if not actor.can_reach_target(task.target.global_position):
		validation_failed.emit()

func _init(actor: Settler, task: Task) -> void:
	super._init(actor, task)
	task = task as DismantleTask
	target = task.target
	constructable = task.target.component_container.get_by_id(Components.Id.Constructable)
	constructable.no_durability_left.connect(func() -> void:
		finished.emit())

func process_action(delta: float) -> void:
	if constructable:
		constructable.reduce_durability(actor.dismantling_speed * delta)
