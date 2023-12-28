extends ActionLeaf

var target: Vector2i

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	#print("Setting target", target)
	actor.set_target(target)
	
	if actor.can_reach_target(target):
		#actor.set_target(null)
		return SUCCESS
		
	# TODO: Check if at target, in which case return finished
	return RUNNING
