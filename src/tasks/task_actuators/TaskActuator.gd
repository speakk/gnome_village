extends Node

class_name TaskActuator

var task: Task

@onready var tree: BeehaveTree = $BeehaveTree as BeehaveTree

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
	task.is_being_worked_on = true
	
func save() -> Dictionary:
	var save_dict := {
		"task_id" = SaveSystem.save_entity(task)
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	task = SaveSystem.get_saved_entity(save_dict["task_id"])

func finish() -> void:
	if task:
		task.is_being_worked_on = false
		task.is_finished = true

func fail() -> void:
	if task:
		task.failed.emit(task)
		task.has_failed = true
		task.is_being_worked_on = false
		task.is_finished = false
