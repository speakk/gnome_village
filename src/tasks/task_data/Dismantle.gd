class_name DismantleTask extends Task

var target: ItemOnGround

func _init() -> void:
	task_id = Tasks.TaskId.Dismantle
	task_name = "Dismantle"

func initialize(params: Dictionary) -> void:
	target = params["target"]
	target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["target_id"] = SaveSystem.save_entity(target)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	target = SaveSystem.get_saved_entity(save_dict["target_id"])
