class_name IsWithinRange extends ConditionLeaf

var target: Node2D

func tick(actor: Node, _blackboard: Blackboard) -> int:
	if not target:
		return FAILURE
	
	var range: float = actor.get_action_range()
	if actor.global_position.distance_to(target) <= range:
		return SUCCESS
	
	return FAILURE
