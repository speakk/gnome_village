class_name GoToUpdatePath extends ActionLeaf

var target_coordinate: Vector2i

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if not blackboard.get_value("path") is PackedVector2Array:
		var map_position_from := Globals.get_map().global_position_to_coordinate(actor.global_position)
		var map_position_to := target_coordinate
		var path := PathFinder.get_id_path_to_closest_point(map_position_from, map_position_to)
		
		if not path or path.size() <= 0:
			return FAILURE
			
		print("Got path", path)
		
		blackboard.set_value("path", path)
		blackboard.set_value("current_path_index", 0)
	
	# TODO: Handle path not finding
	return SUCCESS
