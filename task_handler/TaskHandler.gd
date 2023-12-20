extends Node

class_name TaskHandler

@onready var blueprint_tree_creator := preload("res://task_handler/trees/blueprint_tree.gd").new()

func _ready() -> void:
	Events.blueprint_placed.connect(_blueprint_placed)

var task_trees: Array[TaskTreeBranch] = []

func _blueprint_placed(tile_position: Vector2i, blueprint: Blueprint) -> void:
	var task_tree := blueprint_tree_creator.create_blueprint_task_tree(tile_position, blueprint, get_tree()) as TaskTreeBranch
	task_trees.append(task_tree)

func get_available_settler(task: Variant) -> Settler:
	var settlers := get_tree().get_nodes_in_group("settler") as Array[Node]
	
	var target: Vector2
	
	if "target_tile" in task:
		var target_tile := task.target_tile as Vector2i
		target = Globals.map.map_to_local(target_tile)
	
	if "target" in task:
		target = task.target.global_position
	
	if target:
		var settlers_clone := settlers.duplicate()
		var available := settlers_clone.filter(func(settler: Settler) -> bool:
			return settler.is_available_for_work()
		)
		
		available.sort_custom(func(a: Settler, b: Settler) -> bool:
			return a.global_position.distance_to(target) < b.global_position.distance_to(target)
		)
		
		return available.front()
		
	else:
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
			var next_available_task: Variant = get_next_available_task(task_tree)
			if next_available_task:
				var available_settler := get_available_settler(next_available_task)
				if available_settler:
					available_settler.start_task(next_available_task)
		
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
