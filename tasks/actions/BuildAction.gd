extends ActionLeaf

var target: Blueprint

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	actor.set_build_target(target)
	
	if target.is_finished():
		actor.set_build_target(null)
		return SUCCESS
		
	return RUNNING
