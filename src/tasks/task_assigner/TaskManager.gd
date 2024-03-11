extends Node

@onready var BLUEPRINT_TREE := preload("res://src/tasks/task_assigner/trees/BlueprintTree.gd")
@onready var FEED_PLANTS_TREE := preload("res://src/tasks/task_assigner/trees/FeedPlantsTree.gd")
@onready var HARVEST_PLANT_TREE := preload("res://src/tasks/task_assigner/trees/HarvestPlantTree.gd")
@onready var DISMANTLE_TREE := preload("res://src/tasks/task_assigner/trees/DismantleTree.gd")

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)
	Events.plant.lacks_growth_requirement.connect(_plant_lacks_growth_requirement)
	Events.plant.matured.connect(_harvest_plant)
	Events.dismantle_issued.connect(_dismantle_issued)
	$Tasks.child_entered_tree.connect(_tasks_changed)
	$Tasks.child_exiting_tree.connect(_tasks_changed)

func _tasks_changed(_node: Node) -> void:
	await get_tree().physics_frame
	var tasks: Array[Task]
	tasks.assign($Tasks.get_children())
	Events.tasks_changed.emit(tasks)
			
func _blueprint_placed(tile_position: Vector2i, blueprint: ItemOnGround) -> void:
	var task_tree := BlueprintTree.new(tile_position, blueprint)
	$Tasks.add_child(task_tree)
	
func _plant_lacks_growth_requirement(growth_spot: GrowthSpotComponent) -> void:
	var task_tree := FeedPlantsTree.new(growth_spot)
	$Tasks.add_child(task_tree)

func _harvest_plant(plant: PlantComponent) -> void:
	var task_tree := HarvestPlantTree.new(plant)
	$Tasks.add_child(task_tree)

func _dismantle_issued(item_on_ground: ItemOnGround) -> void:
	var task_tree := DismantleTree.new(item_on_ground)
	$Tasks.add_child(task_tree)

func get_available_settler(task: Variant) -> Settler:
	var settlers := get_tree().get_nodes_in_group("settler") as Array[Node]
	
	var target: Vector3
	
	if "target_tile" in task:
		var target_tile := task.target_tile as Vector2i
		target = Globals.map.coordinate_to_global_position(target_tile)
	
	# TODO: Make "settler picker heuristic" function or whatever
	# for each task type that defines how settlers are prioritized
	if "blueprint" in task:
		target = task.blueprint.global_position
	
	#print("Getting available settler", target)
	
	if target:
		var settlers_clone := settlers.duplicate()
		var available := settlers_clone.filter(func(settler: Settler) -> bool:
			return settler.is_available_for_work()
		)
		
		available.sort_custom(func(a: Settler, b: Settler) -> bool:
			return a.global_position.distance_to(target) < b.global_position.distance_to(target)
		)
		
		if available.size() > 0:
			return available.front()
		
	else:
		for settler in settlers:
			if settler.is_available_for_work():
				return settler
	
	return null

var task_process_timer := 0.0
var task_process_delay := 0.5

enum NodeStatus {
	Unfinished, Finished, FoundTask
}

class NodeResult:
	var task: Task
	var status: NodeStatus
	
	func _init(_task: Task, _status: NodeStatus) -> void:
		task = _task
		status = _status    

# Returns Vector3 or null
func get_approximate_task_location(task: Task) -> Variant:
	var target: Vector3
	
	if "target_tile" in task:
		var target_tile := task.target_tile as Vector2i
		return Globals.map.coordinate_to_global_position(target_tile)
	
	if "blueprint" in task:
		return task.blueprint.global_position
		
	return null

func get_available_task(actor_position: Vector3) -> Task:
	var all_available_tasks: Array[Task]
	for task_tree in $Tasks.get_children() as Array[Task]:
		if task_tree:
			var result: NodeResult = find_unfinished_task_in_tree(task_tree)
			if result.status == NodeStatus.FoundTask:
				var next_available_task: Task = result.task
				all_available_tasks.append(next_available_task)
	
	all_available_tasks.sort_custom(func(a: Task, b: Task) -> bool:
			var a_location: Variant = get_approximate_task_location(a)
			var b_location: Variant = get_approximate_task_location(b)
			
			if not a_location is Vector3:
				return false
			
			if not b_location is Vector3:
				return true
			
			return a_location.distance_to(actor_position) < b_location.distance_to(actor_position)
			)
	
	if all_available_tasks.size() > 0:
		return all_available_tasks.front()
	
	return null

func find_unfinished_task_in_tree(task: Task) -> NodeResult:
	if task.order_type == Task.OrderType.Sequence:
		for sub_task: Task in task.get_subtasks():
			var result: NodeResult = find_unfinished_task_in_tree(sub_task)
			if result.status == NodeStatus.Unfinished:
				return NodeResult.new(null, NodeStatus.Unfinished)
			
			if result.task and result.task.is_leaf():
				var sub_sub_task: Task = result.task
				if sub_sub_task and not sub_sub_task.is_finished:
					if sub_sub_task.is_being_worked_on:
						# Sequence is occupied
						return NodeResult.new(null, NodeStatus.Unfinished)
					return NodeResult.new(sub_sub_task, NodeStatus.FoundTask)
			
		# Node is done
		return NodeResult.new(null, NodeStatus.Finished)
	elif task.order_type == Task.OrderType.Parallel:
		var all_children_finished := true
		for sub_task: Task in task.get_subtasks():
			var result: NodeResult = find_unfinished_task_in_tree(sub_task)
			if result.task and result.task.is_leaf():
				var sub_sub_task: Task = result.task
				if sub_sub_task:
					if sub_sub_task.is_finished:
						continue
					
					if sub_sub_task.is_being_worked_on:
						all_children_finished = false
					else:
						return NodeResult.new(sub_sub_task, NodeStatus.FoundTask)
				# Every task is being done
			else:
				return result
				
		if all_children_finished:
			return NodeResult.new(null, NodeStatus.Finished)
		else:
			return NodeResult.new(null, NodeStatus.Unfinished)
	# Leaf
	return NodeResult.new(task, NodeStatus.FoundTask)
