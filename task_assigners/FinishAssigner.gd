extends ActionLeaf

var assigner: Variant

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	print("Firing assigner finished??")
	Events.task_assigner_finished.emit(assigner)
	return SUCCESS
