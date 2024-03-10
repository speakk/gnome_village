class_name BuildTask extends Task

var constructable_component: ConstructableComponent

func _init() -> void:
	task_id = Tasks.TaskId.Build
	task_name = "Build"

func initialize(params: Dictionary) -> void:
	constructable_component = params["constructable_component"]

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["constructable_component_id"] = SaveSystem.save_entity(constructable_component)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	constructable_component = SaveSystem.get_saved_entity(save_dict["constructable_component_id"])
