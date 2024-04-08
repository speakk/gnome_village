extends Node

class_name TaskActuator

var task: Task
var actor: Settler

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
	print("Start work")
	tree.actor = get_parent()
	tree.enable()
	task.is_being_worked_on = true
	actor = get_parent()
	
func finish() -> void:
	if task:
		task.is_being_worked_on = false
		task.is_finished = true

func fail() -> void:
	if task:
		task.failed.emit()
		task.has_failed = true
		task.is_being_worked_on = false
		task.is_finished = false
		tree.disable()
