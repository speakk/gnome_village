class_name IsWithinRange extends ConditionLeaf

var target: Node2D

func tick(actor: Node, blackboard: Blackboard) -> int:
	var target_coordinate := blackboard.get_value("target_coordinate") as Vector2i
	var target_position := Globals.get_map().coordinate_to_global_position(target_coordinate)
	
	var range: float = actor.get_action_range()
	if actor.global_position.distance_to(target_position) <= range:
		return SUCCESS
	
	return FAILURE
