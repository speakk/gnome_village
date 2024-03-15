class_name IsBlockedByDoor extends ConditionLeaf

#var next_coordinate: Vector2i

signal blocking_door_found(door: DoorComponent)

func tick(actor: Node, blackboard: Blackboard) -> int:
	#assert(next_coordinate)
	#var next_coordinate := actor.get_next_path_coordinate() as Vector2i
	var current_path_index := blackboard.get_value("current_path_index") as int
	var path := blackboard.get_value("path") as PackedVector2Array
	if current_path_index < path.size() - 1:
		var next_coordinate := path[current_path_index + 1]
		var entities := Globals.get_map().get_map_entities(next_coordinate)
		for entity in entities:
			var container: ComponentContainer = entity.component_container
			if container.has_component(Components.Id.Door):
				if not container.has_component(Components.Id.Blueprint):
					var door: DoorComponent = entity.component_container.get_by_id(Components.Id.Door)
					if not door.is_open():
						blocking_door_found.emit(door)
						blackboard.set_value("blocking_door", door)
						print("DID have door in IsBlockedByDoor")
						return SUCCESS
	
	
	#print("No door in IsBlockedByDoor")
	return FAILURE
