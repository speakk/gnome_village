extends Task

class_name DismantleTask

var target: ItemOnGround

func initialize(_target: ItemOnGround) -> DismantleTask:
	target = _target
	
	target.reserved_for_dismantling = true
	
	%GoToAction.target = target.global_position
	print("Set gototarget to be: ", %GoToAction.target)
	%DismantleAction.target = target
	$SequenceComposite/FinishTask.finished.connect(func() -> void:
		Events.dismantle_finished.emit(target)
	)
	
	return self

func tick() -> int:
	return super.tick()

func clean_up() -> void:
	target.reserved_for_dismantling = false
