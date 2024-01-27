extends Node

class_name Task

@onready var tree: BeehaveTree = $BeehaveTree as BeehaveTree
@onready var persistent: Persistent = $Persistent as Persistent

var is_being_worked_on := false:
	set(new_value):
		if new_value:
			if is_inside_tree():
				call_deferred("start_work")
		is_being_worked_on = new_value
var is_finished := false:
	set(new_value):
		if new_value:
			Events.task_finished.emit(self)
		is_finished = new_value
var has_failed := false

func _ready() -> void:
	tree.disable()

func _exit_tree() -> void:
	$BeehaveTree._exit_tree()
	clean_up()

# Implement when extending
func clean_up() -> void:
	pass

func start_work() -> void:
	tree.actor = get_parent()
	tree.enable()

func save() -> Dictionary:
	var save_dict := {
		"is_being_worked_on" = is_being_worked_on,
		"is_finished" = is_finished,
		"has_failed" = has_failed
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	is_being_worked_on = save_dict["is_being_worked_on"]
	is_finished = save_dict["is_finished"]
	has_failed = save_dict["has_failed"]
