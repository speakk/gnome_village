extends Node

class_name NewTaskHandler

##[derive(Component)]
#struct TaskTreeBranch {
	#branch_type: BranchType,
	#order_type: OrderType,
#}
#
##[derive(Component)]
#struct TaskTreeLeaf {
	#task: TaskType,
	#finished: bool,
#}

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)

var task_trees: Array[TaskTreeBranch] = []

func _blueprint_placed(tile_position: Vector2i, blueprint: Blueprint) -> void:
	var task_tree := create_blueprint_task_tree(tile_position, blueprint)
	task_trees.append(task_tree)
	print("Got task tree", task_tree)

func get_available_settler() -> Settler:
	var settlers := get_tree().get_nodes_in_group("settler") as Array[Node]
	
	# TODO: Make it so that this Node gets an optional heuristic on how to pick which settler
	# does this job - for now just pick first settler that is available for work
	for settler in settlers:
		if settler.is_available_for_work():
			return settler
	
	return null

func _process(delta: float) -> void:
	for task_tree in task_trees:
		var next_available_task: Variant = get_next_available_task(task_tree)
		var available_settler := get_available_settler()
		if next_available_task and available_settler:
			available_settler.start_task(next_available_task)
			print("Started task")


var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

enum OrderType {
	Sequence, Parallel
}

enum TaskTreeStatus {
	Initial, Running, Finished, Failed
}

class TaskTreeBranch extends Node:
	var order_type: OrderType = OrderType.Sequence
	var status: TaskTreeStatus = TaskTreeStatus.Initial

class TaskTreeLeaf extends Node:
	var task: Task
	var status: TaskTreeStatus = TaskTreeStatus.Initial

func create_blueprint_task_tree(tile_target: Vector2i, blueprint: Blueprint) -> TaskTreeBranch:
	var blueprint_tree := TaskTreeBranch.new()
	blueprint_tree.order_type = OrderType.Sequence
	
	var building_type := blueprint.building_type
	var material_requirements := BuildingTypes.get_building_requirements(building_type)
	
	var bring_resources := TaskTreeBranch.new()
	bring_resources.order_type = OrderType.Parallel
	
	for material_requirement in material_requirements:
		#var bring_resource_task := 
		var bring_resource_leaf := TaskTreeLeaf.new()
		bring_resource_leaf.task = (BRING_RESOURCE_TASK.instantiate() as BringResourceTask).initialize(tile_target, material_requirement)
		bring_resources.add_child(bring_resource_leaf)
	
	var build_leaf := TaskTreeLeaf.new()
	build_leaf.task = (BUILD_TASK.instantiate() as BuildTask).initialize(blueprint)
	
	blueprint_tree.add_child(bring_resources)
	blueprint_tree.add_child(build_leaf)
	
	return blueprint_tree

func get_next_available_task(object: Variant) -> Variant:
	if object is TaskTreeLeaf:
		return object.task
	
	if object is TaskTreeBranch:
		for child: Variant in (object.get_children() as Array[Variant]):
			var result: Variant = get_next_available_task(child)
			if result is Task:
				if object.order_type == OrderType.Sequence:
					if result.is_being_worked_on and not result.is_finished:
						return null
					if result.is_finished:
						continue
					return result
				
				if object.order_type == OrderType.Parallel:
					if not result.is_being_worked_on:
						return result
	
	return null
