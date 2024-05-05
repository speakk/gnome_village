class_name HarvestPlantTree extends Task

var _plant: PlantComponent

func _init(plant: PlantComponent = null) -> void:
	task_name = "Harvest plant"
	order_type = Task.OrderType.Sequence
	
	if not plant:
		return
	
	_plant = plant
	
	var task := DismantleTask.new({
		target = plant.get_owner()
	})
	
	register_subtask(task)

func _handle_task_failure(_task: Task) -> void:
	print("Harvest failed")

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["_plant_owner_id"] = SaveSystem.get_save_id(_plant.get_owner())
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["_plant_owner_id"], func(plant_owner: Entity) -> void:
		_plant = plant_owner.component_container.get_by_id(Components.Id.Plant)
		))
#endregion
