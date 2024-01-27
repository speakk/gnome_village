class_name BuildTask extends Task

var blueprint: ItemOnGround

func _init() -> void:
	task_id = Tasks.TaskId.Build

func initialize(params: Dictionary) -> void:
	blueprint = params["blueprint"]

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["blueprint_id"] = SaveSystem.save_entity(blueprint)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	blueprint = SaveSystem.get_saved_entity(save_dict["blueprint_id"])
