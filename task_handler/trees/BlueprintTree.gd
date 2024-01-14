extends TaskTree

class_name BlueprintTree

var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

var blueprint: ItemOnGround

func _ready() -> void:
	name = "ItemOnGround_Tree"
	
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
	
func initialize(tile_target: Vector2i, _blueprint: ItemOnGround, scene_tree: SceneTree) -> BlueprintTree:
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
		bring_resource_leaf.task = (BRING_RESOURCE_TASK.instantiate() as BringResourceTask).initialize(tile_target, material_requirement, blueprint, scene_tree)
		bring_resources.add_child(bring_resource_leaf)
	
	var build_leaf := TaskTreeLeaf.new()
	build_leaf.task = (BUILD_TASK.instantiate() as BuildTask).initialize(blueprint)
	build_leaf.name = "Build_Leaf"
	
	add_child(bring_resources)
	add_child(build_leaf)

	return self
