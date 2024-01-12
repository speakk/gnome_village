class_name IsBlockedByDoor extends ConditionLeaf

#var next_coordinate: Vector2i

func tick(actor: Node, blackboard: Blackboard) -> int:
	#assert(next_coordinate)
	#var next_coordinate := actor.get_next_path_coordinate() as Vector2i
	var current_path_index := blackboard.get_value("current_path_index") as int
	var next_coordinate := (blackboard.get_value("path") as PackedVector2Array)[current_path_index + 1]
	var entities := Globals.get_map().get_map_entities(next_coordinate)
	for entity in entities:
		if entity.item.special_features.has(Item.SpecialFeatures.Door):
				return SUCCESS
	
	return FAILURE
