extends Node

var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

func create_blueprint_task_tree(tile_target: Vector2i, blueprint: Blueprint, scene_tree: SceneTree) -> TaskTreeBranch:
	var blueprint_tree := TaskTreeBranch.new()
	blueprint_tree.order_type = TaskTreeBranch.OrderType.Sequence
	blueprint_tree.name = "Blueprint_Tree"
	
	var item_id := blueprint.item_id
	var material_requirements := Items.get_crafting_requirements(item_id)
	
	var bring_resources := TaskTreeBranch.new()
	bring_resources.order_type = TaskTreeBranch.OrderType.Parallel
	bring_resources.name = "Bring_Resources_Parallel"
	
	for material_requirement in material_requirements as Array[ItemRequirement]:
		#var bring_resource_task := 
		var bring_resource_leaf := TaskTreeLeaf.new()
		bring_resource_leaf.name = "Bring_Resource_Leaf"
		bring_resource_leaf.task = (BRING_RESOURCE_TASK.instantiate() as BringResourceTask).initialize(tile_target, material_requirement, blueprint, scene_tree)
		bring_resources.add_child(bring_resource_leaf)
	
	var build_leaf := TaskTreeLeaf.new()
	build_leaf.task = (BUILD_TASK.instantiate() as BuildTask).initialize(blueprint)
	build_leaf.name = "Build_Leaf"
	
	blueprint_tree.add_child(bring_resources)
	blueprint_tree.add_child(build_leaf)
	
	return blueprint_tree
