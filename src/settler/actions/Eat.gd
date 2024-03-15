class_name EatActorAction extends ActorTaskAction

var consumable: ConsumableComponent
var eating_time: float = 2.0

func _init(actor: Settler, task: Task) -> void:
	super._init(actor, task)
	consumable = (task as EatTask).consumable

func process_action(delta: float) -> void:
	if consumable:
		eating_time -= delta
		if eating_time <= 0:
			var character_stats: CharacterStatsComponent = actor.component_container.get_by_id(Components.Id.CharacterStats)
			character_stats.apply_satisfactions(consumable.consume())
			finished.emit()
