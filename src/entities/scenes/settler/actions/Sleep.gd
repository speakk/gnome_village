class_name SleepActorAction extends ActorTaskAction

func _init(actor: Settler, task: Task) -> void:
	super._init(actor, task)
	task = task as SleepTask

func process_action(delta: float) -> void:
	var stats: CharacterStatsComponent = actor.component_container.get_by_id(Components.Id.CharacterStats)
	stats.apply_stat_amount(CharacterStatsComponent.Id.Tiredness, -delta * 0.02)
	if stats.get_stat(CharacterStatsComponent.Id.Tiredness).value < 0.1:
		finished.emit()
