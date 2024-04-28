class_name SimpleActuator extends TaskActuator

func initialize(_task: Task) -> TaskActuator:
	task = _task
	return self

func start_work() -> void:
	super.start_work()
	var target_position: Vector3 = task.get_target(actor)
	if not target_position:
		push_error("There is no target_position for task %s. Missing get_target implementation?" % task.task_name)
	%GoToTarget.target_coordinate = Globals.get_map().global_position_to_coordinate(target_position)
	%DoAction.action = task.create_action(actor)
	%PlayAnimationAction.animation_name = task.animation_name
