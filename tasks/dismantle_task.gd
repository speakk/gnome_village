extends Task

class_name DismantleTask

var target: ItemOnGround:
	set(new_value):
		new_value.reserved_for_dismantling = true
		
		%GoToAction.target_coordinate = Globals.get_map().global_position_to_coordinate(new_value.global_position)
		%DismantleAction.target = new_value
		$SequenceComposite/FinishTask.finished.connect(func() -> void:
			Events.dismantle_finished.emit(new_value)
		)
		
		target = new_value

func initialize(_target: ItemOnGround) -> DismantleTask:
	target = _target
	
	return self

func clean_up() -> void:
	target.reserved_for_dismantling = false


func save() -> Dictionary:
	var save_dict := super.save()
	if target:
		save_dict["target_save_id"] = SaveSystem.get_object_save_id(target)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	if save_dict.has("target_save_id"):
		SaveSystem.register_load_reference(self, "target", save_dict["target_save_id"])
