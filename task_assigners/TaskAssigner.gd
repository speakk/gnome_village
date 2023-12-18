extends SequenceComposite

class_name TaskAssigner

var task: Task

var settler: Settler

func initialize(_task: Task) -> TaskAssigner:
	task = _task
	print("initialized task assigner with task: ", _task)
	return self
