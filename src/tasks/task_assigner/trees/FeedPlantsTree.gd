class_name FeedPlantsTree extends TaskTree

var BRING_RESOURCE_TASK := preload("res://src/tasks/task_data/BringResource.tscn")

var farm_plot: ItemOnGround

func _ready() -> void:
	name = "FeedPlantsTree"
	
func initialize(_farm_plot: ItemOnGround) -> FeedPlantsTree:
	order_type = TaskTreeBranch.OrderType.Parallel
	farm_plot = _farm_plot
	
	var growth_requirements := (_farm_plot.item_scene as FarmPlot).planted_plant.plant.growth_requirements
	
	if growth_requirements.size() > 0:
		var bring_resources := TaskTreeBranch.new()
		bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
		bring_resources.name = "Bring_Resources_Parallel"
		
		for growth_requirement in growth_requirements as Array[ItemRequirement]:
			var bring_resource_leaf := TaskTreeLeaf.new()
			bring_resource_leaf.name = "Bring_Resource_Leaf"
			var task := BRING_RESOURCE_TASK.instantiate() as BringResourceTask
			task.initialize({
				item_requirement = growth_requirement,
				inventory = (_farm_plot.item_scene as FarmPlot).growth_spot.growth_requirement_inventory
			})
			task.failed.connect(_handle_task_failure)
			bring_resource_leaf.set_task(task)
			bring_resources.add_child(bring_resource_leaf)
		
		add_child(bring_resources)
	
	return self

func _handle_task_failure(task: Task) -> void:
	pass

func save() -> Dictionary:
	var save_dict: Dictionary = {}

	var bring_resources_parallel_children_ids: Array[int]
	
	for bring_resources_leaf in get_node("Bring_Resources_Parallel").get_children():
		bring_resources_parallel_children_ids.append(SaveSystem.save_entity(bring_resources_leaf.task))
		
	save_dict["bring_resources_parallel_children_ids"] = bring_resources_parallel_children_ids
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	var bring_resources := TaskTreeBranch.new()
	bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
	bring_resources.name = "Bring_Resources_Parallel"
	
	for id in save_dict["bring_resources_parallel_children_ids"] as Array[int]:
		var bring_resource_leaf := TaskTreeLeaf.new()
		bring_resource_leaf.name = "Bring_Resource_Leaf"
		bring_resources.add_child(bring_resource_leaf)
		bring_resource_leaf.set_task(SaveSystem.get_saved_entity(id))
	
	add_child(bring_resources)
