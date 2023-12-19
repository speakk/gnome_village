extends Node

class_name TaskHandler

@onready var blueprint_tree_creator := preload("res://task_handler/trees/blueprint_tree.gd").new()

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)

var task_trees: Array[TaskTreeBranch] = []

func _blueprint_placed(tile_position: Vector2i, blueprint: Blueprint) -> void:
	var task_tree := blueprint_tree_creator.create_blueprint_task_tree(tile_position, blueprint) as TaskTreeBranch
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

var task_process_timer := 0.0
var task_process_delay := 0.5

func _process(delta: float) -> void:
	task_process_timer += delta
	if task_process_timer >= task_process_delay:
		for task_tree in task_trees:
			print("Trying to get next available task")
			var next_available_task: Variant = get_next_available_task(task_tree)
			var available_settler := get_available_settler()
			if next_available_task and available_settler:
				available_settler.start_task(next_available_task)
				#print("Started task: ", next_available_task)
				print("GOT AND STARTED TASK: ", next_available_task)
		
		task_process_timer = 0


enum TaskTreeStatus {
	Initial, Running, Finished, Failed
}

func get_next_available_task(object: Variant) -> Variant:
	#print("Entering get_next_available task with: ", object.name)
	if object is TaskTreeLeaf:
		#print("Returning task as hit leaf")
		return object.task
	
	if object is TaskTreeBranch:
		#print("Loop through children in: ", object.name, " is order type: ", object.order_type)
		for child: Variant in (object.get_children() as Array[Variant]):
			#print("Going through child in object ", child.name, " parent: ", object.name)
			var result: Variant = get_next_available_task(child)
			
			if result is Task:
				#print("is_being_worked on & finished: ", result.is_being_worked_on, " & ", result.is_finished)
				if object.order_type == TaskTreeBranch.OrderType.Sequence:
					if result.is_finished:
						#print("In sequence, was finished so continue")
						continue
					if result.is_being_worked_on and not result.is_finished:
						#print("In sequence, is being worked on already and not finished, so return null")
						return null
					
					#print("In sequence, had available task in it so returning result: ", result)
					return result
				
				if object.order_type == TaskTreeBranch.OrderType.Parallel:
					if not result.is_being_worked_on:
						#print("Parallel and returning task because not being worked on")
						return result
			elif object.order_type == TaskTreeBranch.OrderType.Sequence:
				#print("Breaking", result)
				break
				
	#print("Hit final null guard")
	return null
