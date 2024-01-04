extends Task

class_name BuildTask

var blueprint: ItemOnGround

func initialize(_blueprint: ItemOnGround) -> BuildTask:
	blueprint = _blueprint
	
	%GoToBlueprint.target = blueprint.position
	%BuildAction.target = blueprint
	
	return self

func tick() -> int:
	return super.tick()
