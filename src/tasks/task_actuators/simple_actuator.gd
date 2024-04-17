class_name SimpleActuator extends TaskActuator

func initialize(_task: Task) -> TaskActuator:
	task = _task
	return self

func start_work() -> void:
	super.start_work()
	%GoToTarget.target_coordinate = Globals.get_map().global_position_to_coordinate(task.get_target(actor))
	%DoAction.action = task.create_action(actor)
	%PlayAnimationAction.animation_name = task.animation_name
