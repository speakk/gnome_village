class_name GoToAction extends SequenceComposite

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS
	
	if not _action:
		print("Alright adding goto action")
		_action = ACTION.new().initialize({ target_coordinate =  target_coordinate })
		_action.finished.connect(func(__action: ActorAction) -> void: _done = true)
		blackboard.set_value("goto_action", _action)
		actor.add_action(_action)
		
	return RUNNING
