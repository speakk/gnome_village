extends TaskActuator

class_name DismantleActuator

func initialize(_task: DismantleTask) -> DismantleActuator:
	task = _task
	
	return self

func start_work() -> void:
	super.start_work()
	
	task.target.reserved_for_dismantling = true
	
	%GoToAction.target_coordinate = Globals.get_map().global_position_to_coordinate(task.target.global_position)
	%DismantleAction.target = task.target
	$SequenceComposite/FinishTask.finished.connect(func() -> void:
		Events.dismantle_finished.emit(task.target)
	)

func clean_up() -> void:
	task.target.reserved_for_dismantling = false

func save() -> Dictionary:
	var save_dict := super.save()
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
