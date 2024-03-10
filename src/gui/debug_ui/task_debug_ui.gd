extends PanelContainer

@onready var debug_ui_tree: Tree = %Tree

func _ready() -> void:
	Events.tasks_changed.connect(_refresh_debug_tree)

func _tasks_changed(_node: Node) -> void:
	_refresh_debug_tree($Tasks.get_children())
	
func _refresh_debug_tree(tasks: Array[Node]) -> void:
	if not debug_ui_tree.visible:
		return
	debug_ui_tree.clear()
	var root := debug_ui_tree.create_item()
	
	for task in tasks:
		if task:
			var child := debug_ui_tree.create_item(root)
			child.set_text(0, task.name)
			
			for subtask in task.get_children():
				var child2 := debug_ui_tree.create_item(child)
				var label := subtask.name
				if subtask is TaskTreeLeaf and not subtask.task:
					continue
				if subtask is TaskTreeLeaf and subtask.task.is_finished:
					label = label + " (DONE)"
					child2.set_custom_color(0, Color.SEA_GREEN)
				elif subtask is TaskTreeBranch:
					var all_finished := false
					for subsubtask in subtask.get_children():
						var child3 := debug_ui_tree.create_item(child2)
						var label3 := subsubtask.name
						if subsubtask is TaskTreeLeaf:
							if not subsubtask.task:
								continue
							if subsubtask.task.is_finished:
								label3 = label3 + " (DONE)"
								child2.set_custom_color(0, Color.SEA_GREEN)
							else:
								all_finished = false
						
						child3.set_text(0, label3)
							
				child2.set_text(0, label)
	
	#root.uncollapse_tree()
