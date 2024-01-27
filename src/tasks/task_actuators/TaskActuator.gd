extends Node

class_name TaskActuator

var task: Task

@onready var tree: BeehaveTree = $BeehaveTree as BeehaveTree
@onready var persistent: Persistent = $Persistent as Persistent

func _ready() -> void:
	tree.disable()

#func _enter_tree() -> void:
	##call_deferred("start_work")
	#start_work()

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
	task.is_being_worked_on = false
	task.is_finished = true
