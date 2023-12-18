extends ActionLeaf

var target: Vector2i

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	actor.set_target(target)
	
	if actor.is_next_to_target(target):
		return SUCCESS
		
	# TODO: Check if at target, in which case return finished
	return RUNNING
