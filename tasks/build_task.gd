extends Task

class_name BuildTask

var blueprint: ItemOnGround

func initialize(_blueprint: ItemOnGround) -> BuildTask:
	blueprint = _blueprint
	
	%GoToBlueprint.target_coordinate = Globals.get_map().global_position_to_coordinate(blueprint.global_position)
	%BuildAction.target = blueprint
	
	return self
