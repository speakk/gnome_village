extends Task

class_name BuildTask

var blueprint: ItemOnGround:
	set(new_value):
		%GoToBlueprint.target_coordinate = Globals.get_map().global_position_to_coordinate(new_value.global_position)
		%BuildAction.target = new_value
		blueprint = new_value

func initialize(_blueprint: ItemOnGround) -> BuildTask:
	blueprint = _blueprint
	
	return self

func save() -> Dictionary:
	var save_dict := super.save()
	if blueprint:
		save_dict["blueprint_save_id"] = SaveSystem.save_entity(blueprint)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	blueprint = SaveSystem.get_saved_entity(save_dict["blueprint_save_id"])
	print(blueprint)
	#if save_dict.has("blueprint_save_id"):
		#SaveSystem.register_load_reference(self, "blueprint", save_dict["blueprint_save_id"])
