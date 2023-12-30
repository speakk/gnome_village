extends Task

class_name DismantleTask

var target: ItemOnGround

func initialize(_target: ItemOnGround) -> DismantleTask:
	target = _target
	
	target.reserved_for_dismantling = true
	
	%GoToAction.target = Globals.get_map().global_to_coordinate(target.global_position)
	%DismantleAction.target = target
	
	return self

func tick() -> int:
	return super.tick()

func clean_up() -> void:
	target.reserved_for_dismantling = false
