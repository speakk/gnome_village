class_name BringResourceTask extends Task

var target_coordinate: Vector2i
var item_requirement: ItemRequirement
var inventory: Inventory

func _init() -> void:
	task_id = Tasks.TaskId.BringResource

func initialize(params: Dictionary) -> void:
	if params.has("target_coordinate"):
		target_coordinate = params["target_coordinate"]
	else:
		inventory = params["inventory"]
		
	item_requirement = params["item_requirement"]
	
	assert(target_coordinate != null or inventory)
	assert(item_requirement)

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["target_coordinate.x"] = target_coordinate.x
	save_dict["target_coordinate.y"] = target_coordinate.y
	save_dict["item_requirement_id"] = SaveSystem.save_entity(item_requirement)
	save_dict["inventory_id"] = SaveSystem.save_entity(inventory)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	inventory = SaveSystem.get_saved_entity(save_dict["item_requirement_id"])
	target_coordinate = Vector2i(save_dict["target_coordinate.x"], save_dict["target_coordinate.y"])
	item_requirement = SaveSystem.get_saved_entity(save_dict["item_requirement_id"])
