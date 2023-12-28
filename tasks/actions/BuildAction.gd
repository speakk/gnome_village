extends ActionLeaf

var target: Blueprint

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if not actor.can_reach_target(target.global_position):
		return FAILURE
	
	actor.set_build_target(target)
	
	if target.is_finished():
		return SUCCESS
		
	return RUNNING
