class_name DismantleTask extends Task

var target: ItemOnGround

func _init() -> void:
	task_id = Tasks.TaskId.Dismantle

func initialize(params: Dictionary) -> void:
	target = params["target"]

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["target_id"] = SaveSystem.save_entity(target)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	target = SaveSystem.get_saved_entity(save_dict["target_id"])
