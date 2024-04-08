extends ActionLeaf

class_name FailTask

@export var fail_message: String = ""

signal failed

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	print("Task failed, not noice: %s" % fail_message)
	#actor.task_handler.fail_current_task()
	(blackboard.get_value("actuator") as TaskActuator).fail()
	#failed.emit()
	return FAILURE
