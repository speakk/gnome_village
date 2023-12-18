extends BeehaveTree

class_name Task

var is_being_worked_on := false
var is_finished := false

func _ready() -> void:
	Events.task_finished.connect(_task_finished)
	
func _task_finished(task: Task) -> void:
	if task == self:
		is_finished = true
