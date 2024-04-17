class_name ActorTaskAction extends ActorAction

func _init(actor: Settler, task: Task) -> void:
	super._init(actor)
	_started = true
	validate_task(actor, task)
	task.cancelled.connect(func() -> void:
		cancelled.emit()
		)

func validate_task(actor: Settler, task: Task) -> void:
	pass
