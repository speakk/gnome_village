extends ActionLeaf

class_name FailTask

signal failed

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	print("Task failed, not noice")
	#actor.task_handler.fail_current_task()
	failed.emit()
	return FAILURE
