extends TaskActuator

class_name DismantleActuator

func initialize(_task: DismantleTask) -> DismantleActuator:
	task = _task
	
	return self

func start_work() -> void:
	super.start_work()
	
	%GoToAction.target_coordinate = Globals.get_map().global_position_to_coordinate(task.target.global_position)
	%DismantleAction.target = task.target
	%FinishTask.finished.connect(func() -> void:
		task.is_finished = true
		finish()
		#Events.dismantle_finished.emit(task.target)
	)

# TODO: I guess this should really be in the Task itself now
func clean_up() -> void:
	if task.target:
		if task.target and task.target.component_container.has_component(Components.Id.Constructable):
			task.target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = false

func save() -> Dictionary:
	var save_dict := super.save()
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
