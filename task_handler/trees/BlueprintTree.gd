extends TaskTree

class_name BlueprintTree

var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

var blueprint: ItemOnGround

func _ready() -> void:
	name = "BlueprintTree"
	
	Events.construction_finished.connect(func(_blueprint: ItemOnGround) -> void:
		if _blueprint == blueprint:
			print("Cleaning up after finishing contruction")
			clean_up()
			#blueprint.call_deferred("queue_free")
	)
	
	Events.blueprint_cancel_issued.connect(func(_blueprint: ItemOnGround) -> void:
		if _blueprint == blueprint:
			clean_up()
			blueprint.call_deferred("queue_free")
	)
	
func initialize(tile_target: Vector2i, _blueprint: ItemOnGround) -> BlueprintTree:
	order_type = TaskTreeBranch.OrderType.Sequence
	blueprint = _blueprint
	
	var item_id := blueprint.item_id
	var material_requirements := Items.get_crafting_requirements(item_id)
	
	var bring_resources := TaskTreeBranch.new()
	bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
	bring_resources.name = "Bring_Resources_Parallel"
	
	for material_requirement in material_requirements as Array[ItemRequirement]:
		#var bring_resource_task := 
		var bring_resource_leaf := TaskTreeLeaf.new()
		bring_resource_leaf.name = "Bring_Resource_Leaf"
		bring_resource_leaf.set_task(Tasks.TaskId.BringResource, {
			tile_target: tile_target, material_requirement: material_requirement, blueprint: blueprint
		})
		#bring_resource_leaf.set_task(BRING_RESOURCE_TASK.instantiate())
		bring_resources.add_child(bring_resource_leaf)
		bring_resource_leaf.task.call_deferred("initialize", tile_target, material_requirement, blueprint)
	
	var build_leaf := TaskTreeLeaf.new()
	build_leaf.set_task(BUILD_TASK.instantiate() as BuildTask)
	build_leaf.name = "Build_Leaf"
	
	add_child(bring_resources)
	add_child(build_leaf)
	
	build_leaf.task.initialize(blueprint)

	return self

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
	
	#SaveSystem.register_load_reference(self, "blueprint", save_dict["blueprint_id"])
	blueprint = SaveSystem.get_saved_entity(save_dict["blueprint_id"])
	
	#SaveSystem.register_load_reference(build_leaf, "task", save_dict["Build_Leaf_Task_Id"])
	
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
	
