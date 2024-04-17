class_name DoAction extends ActionLeaf

var params: Dictionary
var _done := false
var _failed := false

var action: ActorAction:
	set(new_value):
		action = new_value
		action.finished.connect(func() ->void:
			_done = true)
		action.cancelled.connect(func() -> void:
			_failed = true
			)
		action.validation_failed.connect(func() -> void:
			_failed = true)

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS
	
	if _failed:
		return FAILURE
	
	
	if not actor.has_action(action):
		actor.add_action(action)
		
	return RUNNING
