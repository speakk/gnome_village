class_name BringResourceTask extends Task

var target_coordinate: Vector2i
var item_requirement: ItemRequirement
var inventory_component: InventoryComponent

func _init(params: Variant = null) -> void:
	task_id = Tasks.TaskId.BringResource
	task_name = "Bring resource"
	task_actuator_scene = preload("res://src/tasks/task_actuators/bring_resource.tscn")
	
	if not params is Dictionary:
		return
	
	if params.has("target_coordinate"):
		target_coordinate = params["target_coordinate"]
	else:
		inventory_component = params["inventory_component"]
		inventory_component.removed.connect(func() -> void:
			is_cancelled = true
			)
		
	item_requirement = params["item_requirement"]
	
	assert(target_coordinate != null or inventory_component)
	assert(item_requirement)


#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["target_coordinate.x"] = target_coordinate.x
	dict["target_coordinate.y"] = target_coordinate.y
	dict["item_requirement"] = item_requirement.serialize()
	if inventory_component:
		dict["inventory_owner_id"] = SaveSystem.get_save_id(inventory_component.get_owner())
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	target_coordinate = Vector2i(dict["target_coordinate.x"], dict["target_coordinate.y"])
	item_requirement = ItemRequirement.new()
	item_requirement.deserialize(dict["item_requirement"])
	if dict.has("inventory_owner_id"):
		SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["inventory_owner_id"], func(inv_owner: Entity) -> void:
			# TODO: Oof no
			inventory_component = inv_owner.component_container.get_by_id(Components.Id.Constructable)._inventory
			))
#endregion
