extends TaskActuator

class_name BuildActuator

func initialize(_task: BuildTask) -> BuildActuator:
	task = _task
	
	return self

func start_work() -> void:
	super.start_work()
	%GoToBlueprint.target_coordinate = Globals.get_map().global_position_to_coordinate(task.blueprint.global_position)
	%BuildAction.target = task.blueprint

func save() -> Dictionary:
	var save_dict := super.save()
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	#if save_dict.has("blueprint_save_id"):
		#SaveSystem.register_load_reference(self, "blueprint", save_dict["blueprint_save_id"])
