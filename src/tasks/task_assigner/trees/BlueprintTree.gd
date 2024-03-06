extends TaskTree

class_name BlueprintTree

var BRING_RESOURCE_TASK := preload("res://src/tasks/task_data/BringResource.tscn")
var BUILD_TASK := preload("res://src/tasks/task_data/Build.tscn")

var blueprint: ItemOnGround

func _ready() -> void:
	name = "BlueprintTree"
	#
	#Events.construction_finished.connect(func(_blueprint: ItemOnGround) -> void:
		#if _blueprint == blueprint:
			#print("Cleaning up after finishing contruction")
			#clean_up()
	#)
	
	Events.blueprint_cancel_issued.connect(func(_blueprint: ItemOnGround) -> void:
		if _blueprint == blueprint:
			clean_up()
			#blueprint.queue_free()
	)

func finish_tree() -> void:
	clean_up()

func initialize(tile_target: Vector2i, _blueprint: ItemOnGround) -> BlueprintTree:
	order_type = TaskTreeBranch.OrderType.Sequence
	blueprint = _blueprint
	
	var item_id := blueprint.item.item_id
	var constructable_component: ConstructableComponent = _blueprint.component_container.get_by_id(Components.Id.Constructable)
	var material_requirements := constructable_component.requirements
	
	if material_requirements.size() > 0:
		var bring_resources := TaskTreeBranch.new()
		bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
		bring_resources.name = "Bring_Resources_Parallel"
		
		# TODO: Each amount gets split into 1
		# Figure if we want to support item stacks being delivered
		for material_requirement in material_requirements as Array[ItemRequirement]:
			for i in material_requirement.amount:
				var bring_resource_leaf := TaskTreeLeaf.new()
				bring_resource_leaf.name = "Bring_Resource_Leaf"
				var task := BRING_RESOURCE_TASK.instantiate() as BringResourceTask
				var requirement := ItemRequirement.new()
				requirement.item_id = material_requirement.item_id
				requirement.amount = 1
				task.initialize({
					item_requirement = requirement,
					inventory_component = constructable_component.get_inventory()
				})
				task.failed.connect(_handle_task_failure)
				bring_resource_leaf.set_task(task)
				bring_resources.add_child(bring_resource_leaf)
		
		add_child(bring_resources)
	
	var build_leaf := TaskTreeLeaf.new()
	var build_task := BUILD_TASK.instantiate() as BuildTask
	build_task.initialize({
		constructable_component = constructable_component
	})
	build_task.failed.connect(_handle_task_failure)
	build_leaf.set_task(build_task)
	build_leaf.name = "Build_Leaf"
	
	add_child(build_leaf)

	return self

func _handle_task_failure(task: Task) -> void:
	pass

func save() -> Dictionary:
	var save_dict: Dictionary = {}
	
	save_dict["Build_Leaf_Task_Id"] = SaveSystem.save_entity(get_node("Build_Leaf").task)
	save_dict["blueprint_id"] = SaveSystem.save_entity(blueprint)
	
	var bring_resources_parallel_children_ids: Array[int]
	
	for bring_resources_leaf in get_node("Bring_Resources_Parallel").get_children():
		bring_resources_parallel_children_ids.append(SaveSystem.save_entity(bring_resources_leaf.task))
		
	save_dict["bring_resources_parallel_children_ids"] = bring_resources_parallel_children_ids
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	blueprint = SaveSystem.get_saved_entity(save_dict["blueprint_id"])
	
	var bring_resources := TaskTreeBranch.new()
	bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
	bring_resources.name = "Bring_Resources_Parallel"
	
	for id in save_dict["bring_resources_parallel_children_ids"] as Array[int]:
		var bring_resource_leaf := TaskTreeLeaf.new()
		bring_resource_leaf.name = "Bring_Resource_Leaf"
		bring_resources.add_child(bring_resource_leaf)
		bring_resource_leaf.set_task(SaveSystem.get_saved_entity(id))
	
	add_child(bring_resources)
	
	var build_leaf := TaskTreeLeaf.new()
	build_leaf.name = "Build_Leaf"
	add_child(build_leaf)
	build_leaf.set_task(SaveSystem.get_saved_entity(save_dict["Build_Leaf_Task_Id"]))
	
