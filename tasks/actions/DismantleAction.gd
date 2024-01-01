extends ActionLeaf

class_name DismantleAction

var target: ItemOnGround

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if not actor.can_reach_target(target.global_position):
		return FAILURE
	
	actor.set_task_target(target)
	
	if not target.has_durability_left():
		return SUCCESS
		
	return RUNNING
