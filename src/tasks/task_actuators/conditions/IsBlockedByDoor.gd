class_name IsBlockedByDoor extends ConditionLeaf

#var next_coordinate: Vector2i

func tick(actor: Node, blackboard: Blackboard) -> int:
	#assert(next_coordinate)
	#var next_coordinate := actor.get_next_path_coordinate() as Vector2i
	var current_path_index := blackboard.get_value("current_path_index") as int
	var path := blackboard.get_value("path") as PackedVector2Array
	if current_path_index < path.size() - 1:
		var next_coordinate := path[current_path_index + 1]
		var entities := Globals.get_map().get_map_entities(next_coordinate)
		for entity in entities:
			if entity.component_container.has_component(Components.Id.Door) and entity.current_state != ItemOnGround.ItemState.Blueprint and not entity.item_scene.is_open():
				blackboard.set_value("blocking_door", entity)
				print("DID have door in IsBlockedByDoor")
				return SUCCESS
	
	
	#print("No door in IsBlockedByDoor")
	return FAILURE
