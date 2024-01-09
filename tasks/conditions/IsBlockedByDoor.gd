class_name IsBlockedByDoor extends ConditionLeaf

var next_coordinate: Vector2i

func tick(actor: Node, _blackboard: Blackboard) -> int:
	if not next_coordinate:
		printerr("IsBlockedByDoor has no next_coordinate")
		return FAILURE
		
	var entities := Globals.get_map().get_map_entities(next_coordinate)
	for entity in entities:
		if entity.item.special_features.has(Item.SpecialFeatures.Door):
				return SUCCESS
	
	return FAILURE
