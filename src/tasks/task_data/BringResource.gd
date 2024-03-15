class_name BringResourceTask extends Task

var target_coordinate: Vector2i
var item_requirement: ItemRequirement
var inventory_component: InventoryComponent

func _init(params: Dictionary) -> void:
	task_id = Tasks.TaskId.BringResource
	task_name = "Bring resource"
	task_actuator_scene = preload("res://src/tasks/task_actuators/bring_resource.tscn")
	
	if params.has("target_coordinate"):
		target_coordinate = params["target_coordinate"]
	else:
		inventory_component = params["inventory_component"]
		
	item_requirement = params["item_requirement"]
	
	assert(target_coordinate != null or inventory_component)
	assert(item_requirement)

func save() -> Dictionary:
	var save_dict: Dictionary = super.save()
	save_dict["target_coordinate.x"] = target_coordinate.x
	save_dict["target_coordinate.y"] = target_coordinate.y
	save_dict["item_requirement_id"] = SaveSystem.save_entity(item_requirement)
	save_dict["inventory_component_id"] = SaveSystem.save_entity(inventory_component)
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	inventory_component = SaveSystem.get_saved_entity(save_dict["inventory_component_id"])
	target_coordinate = Vector2i(save_dict["target_coordinate.x"], save_dict["target_coordinate.y"])
	item_requirement = SaveSystem.get_saved_entity(save_dict["item_requirement_id"])
