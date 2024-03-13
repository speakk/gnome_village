class_name EatActuator extends TaskActuator

func initialize(_task: EatTask) -> EatActuator:
	task = _task
	return self

func start_work() -> void:
	super.start_work()
	
	%GoToAction.target_coordinate = Globals.get_map().global_position_to_coordinate(task.target.global_position)
	%DismantleAction.target = task.target
	%FinishTask.finished.connect(func() -> void:
		task.is_finished = true
		finish()
	)
	
	%DoAction.action = EatActorAction.new(actor, {
		consumable = task.consumable
	})
