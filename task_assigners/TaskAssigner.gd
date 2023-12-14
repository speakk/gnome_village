extends SequenceComposite

class_name TaskAssigner

@export var task: Task

var settler: Settler

func initialize(_task: Task = null) -> TaskAssigner:
	task = _task
	return self
