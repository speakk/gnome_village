class_name HarvestPlantTree extends Task

var _plant: PlantComponent

func _init(plant: PlantComponent) -> void:
	task_name = "Harvest plant"
	order_type = Task.OrderType.Sequence
	_plant = plant
	
	var task := DismantleTask.new({
		target = plant.get_owner()
	})
	
	register_subtask(task)

func _handle_task_failure(task: Task) -> void:
	print("Harvest failed")
