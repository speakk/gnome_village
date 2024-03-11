extends Node

@onready var BLUEPRINT_TREE := preload("res://src/tasks/task_assigner/trees/BlueprintTree.gd")
@onready var FEED_PLANTS_TREE := preload("res://src/tasks/task_assigner/trees/FeedPlantsTree.gd")
@onready var HARVEST_PLANT_TREE := preload("res://src/tasks/task_assigner/trees/HarvestPlantTree.gd")
@onready var DISMANTLE_TREE := preload("res://src/tasks/task_assigner/trees/DismantleTree.gd")

#@onready var debug_ui_tree := %Tree as Tree

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)
	Events.plant.lacks_growth_requirement.connect(_plant_lacks_growth_requirement)
	Events.plant.matured.connect(_harvest_plant)
	Events.dismantle_issued.connect(_dismantle_issued)
	$Tasks.child_entered_tree.connect(_tasks_changed)
	$Tasks.child_exiting_tree.connect(_tasks_changed)
	#Events.task_finished.connect(func(_task: Task) -> void: _refresh_debug_tree($Tasks.get_children()))
	#Events.debug_visuals_set.connect(func(new_value: bool) -> void:
		#if new_value:
			#_refresh_debug_tree($Tasks.get_children())
		#%DebugUI.visible = new_value
	#)

func _tasks_changed(_node: Node) -> void:
	Events.tasks_changed.emit($Tasks.get_children())
	#_refresh_debug_tree($Tasks.get_children())
			
func _blueprint_placed(tile_position: Vector2i, blueprint: ItemOnGround) -> void:
	var task_tree := (BLUEPRINT_TREE.new() as BlueprintTree) as TaskTreeBranch
	$Tasks.add_child(task_tree)
	task_tree.initialize(tile_position, blueprint)
	
func _plant_lacks_growth_requirement(growth_spot: GrowthSpotComponent) -> void:
	var task_tree := FEED_PLANTS_TREE.new() as FeedPlantsTree
	$Tasks.add_child(task_tree)
	task_tree.initialize(growth_spot)

func _harvest_plant(plant: PlantComponent) -> void:
	var task_tree := HARVEST_PLANT_TREE.new() as HarvestPlantTree
	$Tasks.add_child(task_tree)
	task_tree.initialize(plant)

func _dismantle_issued(item_on_ground: ItemOnGround) -> void:
	var task_tree := (DISMANTLE_TREE.new() as DismantleTree)
	$Tasks.add_child(task_tree)
	task_tree.initialize(item_on_ground)

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

#func _process(delta: float) -> void:
	#task_process_timer += delta
	#if task_process_timer >= task_process_delay:
		##for task_tree in task_trees:
		#for task_tree in $Tasks.get_children() as Array[TaskTreeBranch]:
			#if task_tree:
				#var result: NodeResult = give_task(task_tree)
				#if result.status == NodeStatus.FoundTask:
					#var next_available_task: Task = result.node.task
					#var available_settler := get_available_settler(next_available_task)
					#if available_settler:
						#next_available_task.is_being_worked_on = true
						#available_settler.start_task(next_available_task)
				#elif result.status == NodeStatus.Finished:
					#task_tree.finish_tree()
		#task_process_timer = 0

enum NodeStatus {
	Unfinished, Finished, FoundTask
}

class NodeResult:
	var node: Variant
	var status: NodeStatus
	
	func _init(_node: Variant, _status: NodeStatus) -> void:
		node = _node
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
	for task_tree in $Tasks.get_children() as Array[TaskTreeBranch]:
		if task_tree:
			var result: NodeResult = give_task(task_tree)
			if result.status == NodeStatus.FoundTask:
				var next_available_task: Task = result.node.task
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

func give_task(node: Variant) -> NodeResult:
	if node is TaskTreeBranch and node.order_type == TaskTreeBranch.OrderType.Sequence:
		for sub_node: Variant in node.get_children():
			var result: Variant = give_task(sub_node)
			if result.status == NodeStatus.Unfinished:
				return NodeResult.new(null, NodeStatus.Unfinished)
			
			if result.node is TaskTreeLeaf:
				var task: Variant = result.node.task
				if task and not task.is_finished:
					if task.is_being_worked_on:
						# Sequence is occupied
						return NodeResult.new(null, NodeStatus.Unfinished)
					return NodeResult.new(result.node, NodeStatus.FoundTask)
			
		# Node is done
		return NodeResult.new(null, NodeStatus.Finished)
	elif node is TaskTreeBranch and node.order_type == TaskTreeBranch.OrderType.Parallel:
		var all_children_finished := true
		for sub_node: Variant in node.get_children():
			var result: NodeResult = give_task(sub_node)
			if result.node is TaskTreeLeaf:
				var task: Variant = result.node.task
				if task:
					if task.is_finished:
						continue
					
					if task.is_being_worked_on:
						all_children_finished = false
					else:
						return NodeResult.new(result.node, NodeStatus.FoundTask)
				# Every task is being done
			else:
				return result
				
		if all_children_finished:
			return NodeResult.new(null, NodeStatus.Finished)
		else:
			return NodeResult.new(null, NodeStatus.Unfinished)
	# Leaf
	return NodeResult.new(node, NodeStatus.FoundTask)
