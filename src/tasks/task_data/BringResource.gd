class_name BringResourceTask extends Task

var target_coordinate: Vector2i
var item_requirement: ItemRequirement
var inventory_holder_entity: Variant

func _init() -> void:
	task_id = Tasks.TaskId.BringResource

func initialize(params: Dictionary) -> void:
	target_coordinate = params["target_coordinate"]
	item_requirement = params["item_requirement"]
	inventory_holder_entity = params["inventory_holder_entity"]
	
	assert(target_coordinate)
	assert(item_requirement)
	assert(inventory_holder_entity)

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["target_coordinate.x"] = target_coordinate.x
	save_dict["target_coordinate.y"] = target_coordinate.y
	save_dict["item_requirement_id"] = SaveSystem.save_entity(item_requirement)
	save_dict["inventory_holder_entity_id"] = SaveSystem.save_entity(inventory_holder_entity)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	target_coordinate = Vector2i(save_dict["target_coordinate.x"], save_dict["target_coordinate.y"])
	inventory_holder_entity = SaveSystem.get_saved_entity(save_dict["inventory_holder_entity_id"])
	item_requirement = SaveSystem.get_saved_entity(save_dict["item_requirement_id"])
