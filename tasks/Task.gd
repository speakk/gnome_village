extends BeehaveTree

class_name Task

var is_being_worked_on := false
var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
		is_finished = new_value
var has_failed := false

func _exit_tree() -> void:
	clean_up()

# Implement when extending
func clean_up() -> void:
	pass
