extends Node

class_name Task

@onready var tree: BeehaveTree = $BeehaveTree as BeehaveTree

var is_being_worked_on := false
var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
		is_finished = new_value
var has_failed := false

func _ready() -> void:
	tree.actor = get_parent()

func _exit_tree() -> void:
	$BeehaveTree._exit_tree()
	clean_up()

# Implement when extending
func clean_up() -> void:
	pass
