extends ActionLeaf

class_name FinishTask

signal finished

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	print("Task finished, noice")
	actor.task_handler.finish_current_task()
	finished.emit()
	return SUCCESS
