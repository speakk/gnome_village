extends Task

class_name BuildTask

var blueprint: Blueprint

func initialize(_blueprint: Blueprint) -> BuildTask:
	blueprint = _blueprint
	
	%GoToBlueprint.target = blueprint.position
	%BuildAction.target = blueprint
	
	return self

func tick() -> int:
	print("BUILD TICKING")
	return super.tick()
