class_name GoToAction extends SequenceComposite

signal failed

var target_coordinate: Vector2i:
	set(new_value):
		%UpdatePath.target_coordinate = new_value
		target_coordinate = new_value

func before_run(actor: Node, blackboard: Blackboard) -> void:
	%IsBlockedByDoor.blocking_door_found.connect(func(door: DoorComponent) -> void:
		%DoActionOpenDoor.action = OpenDoorActorAction.new(actor, {
			door = door
		})
		)
	%FailToFindPath.failed.connect(func() -> void:
		failed.emit()
		)

@warning_ignore("untyped_declaration")
func tick(_node, _blackboard) -> int:
	#print("Ticking GoToAction, ", name)
	return super.tick(_node, _blackboard)
