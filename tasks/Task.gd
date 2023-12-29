extends BeehaveTree

class_name Task

var is_being_worked_on := false
var is_finished := false
var has_failed := false

func _exit_tree() -> void:
	clean_up()

# Implement when extending
func clean_up() -> void:
	pass
