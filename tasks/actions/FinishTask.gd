extends ActionLeaf

class_name FinishTask

var target: Blueprint

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	actor.finish_current_task()
	return SUCCESS
