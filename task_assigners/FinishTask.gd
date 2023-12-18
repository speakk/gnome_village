extends ActionLeaf

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	print("Task finished wot")
	Events.task_finished.emit(get_parent().task)
	return SUCCESS
