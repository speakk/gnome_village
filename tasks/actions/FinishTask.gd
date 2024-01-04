extends ActionLeaf

class_name FinishTask

var target: ItemOnGround

signal finished

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	print("Task finished, noice")
	actor.finish_current_task()
	finished.emit()
	return SUCCESS
