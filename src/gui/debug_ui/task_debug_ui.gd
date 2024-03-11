extends PanelContainer

@onready var debug_ui_tree: Tree = %Tree

var _cached_tasks: Array[Task]

func _ready() -> void:
	Events.tasks_changed.connect(_refresh_debug_tree)
	Events.debug_visuals_set.connect(func(new_value: bool) -> void:
		if new_value:
			_refresh_debug_tree(_cached_tasks)
		visible = new_value
)
	
func _refresh_debug_tree(tasks: Array[Task]) -> void:
	_cached_tasks = tasks
	if not debug_ui_tree.visible:
		return
	debug_ui_tree.clear()
	var root := debug_ui_tree.create_item()
	
	for task in tasks:
		if task:
			var child := debug_ui_tree.create_item(root)
			child.set_text(0, task.task_name)
			child.collapsed = true
			
			for subtask: Task in task.get_subtasks():
				var child2 := debug_ui_tree.create_item(child)
				var label: String = subtask.task_name
				if subtask.is_leaf() and subtask.is_finished:
					label = label + " (DONE)"
					child2.set_custom_color(0, Color.SEA_GREEN)
				elif not subtask.is_leaf():
					var all_finished := false
					for subsubtask: Task in subtask.get_subtasks():
						var child3 := debug_ui_tree.create_item(child2)
						var label3: String = subsubtask.task_name
						if subsubtask.is_leaf():
							if subsubtask.is_finished:
								label3 = label3 + " (DONE)"
								child2.set_custom_color(0, Color.SEA_GREEN)
							else:
								all_finished = false
						
						child3.set_text(0, label3)
							
				child2.set_text(0, label)
	
	#root.uncollapse_tree()
	#root.collapse_tree()
	#root.collapsed = true
