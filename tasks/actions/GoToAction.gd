class_name GoToAction extends SequenceComposite

var target_coordinate: Vector2i:
	set(new_value):
		%UpdatePath.target_coordinate = new_value
		%IsWithinRange.target_coordinate = new_value
		target_coordinate = new_value

@warning_ignore("untyped_declaration")
func tick(_node, _blackboard) -> int:
	print("Ticking GoToAction, ", name)
	return super.tick(_node, _blackboard)
