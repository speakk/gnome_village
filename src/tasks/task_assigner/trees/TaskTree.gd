extends TaskTreeBranch

class_name TaskTree

var _unfinished_subtasks: int = 0

func register_subtask(task: Task) -> void:
	_unfinished_subtasks += 1
	task.finished.connect(func() -> void:
		_unfinished_subtasks -= 1
		if _unfinished_subtasks <= 0:
			clean_up()
		)
