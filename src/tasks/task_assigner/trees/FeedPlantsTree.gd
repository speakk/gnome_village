class_name FeedPlantsTree extends Task

var _growth_spot: GrowthSpotComponent

func _init(growth_spot: GrowthSpotComponent) -> void:
	task_name = "Tend plants"
	name = "FeedPlantsTree"

	order_type = Task.OrderType.Parallel
	_growth_spot = growth_spot
	var plant_component: PlantComponent = growth_spot.plant_component
	var growth_requirements := plant_component.growth_requirements
	
	if growth_requirements.size() > 0:
		var bring_resources := Task.new()
		bring_resources.order_type = Task.OrderType.Parallel
		bring_resources.name = "Bring_Resources_Parallel"
		
		for growth_requirement in growth_requirements as Array[ItemRequirement]:
			for i in growth_requirement.amount:
				var split_requirement := growth_requirement.duplicate()
				split_requirement.amount = 1
				var task := BringResourceTask.new({
					item_requirement = split_requirement,
					inventory_component = growth_spot.growth_requirement_inventory
				})
				task.failed.connect(_handle_task_failure)
				bring_resources.register_subtask(task)
		
		register_subtask(bring_resources)

func _handle_task_failure() -> void:
	print("Feed plants bring task failed")
