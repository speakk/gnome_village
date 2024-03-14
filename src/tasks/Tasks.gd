extends Node

enum TaskId {
	BringResource, Build, Dismantle, Eat
}

var task_actuators: Dictionary = {
	TaskId.BringResource: preload("res://src/tasks/task_actuators/bring_resource.tscn"),
	TaskId.Build: preload("res://src/tasks/task_actuators/build.tscn"),
	TaskId.Dismantle: preload("res://src/tasks/task_actuators/dismantle.tscn"),
	TaskId.Eat: preload("res://src/tasks/task_actuators/eat.tscn"),
}

# TODO: Can't really auto-map these to an id dictionary, as we shouldn't really preload scenes, hrm
#var actuator_by_id: Dictionary
#func _ready() -> void:
	#var data_dir := DirAccess.open("res://src/tasks/task_actuators")
	#data_dir.list_dir_begin()
	#var file_name := data_dir.get_next()
	#while file_name != "":
		#if not data_dir.current_is_dir():
			#var data := load("res://src/tasks/task_actuators/%s" % file_name)
			#var actuator: PackedScene
			#if data is PackedScene:
				#actuator = data
				#if actuator_by_id.has(component_data.id):
					#push_error("Component Id duplicate found: ", component_data.id, file_name)
				#component_by_id[component_data.id] = component_data
		#file_name = data_dir.get_next()


func create_task_actuator(task: Task) -> TaskActuator:
	var actuator := task_actuators[task.task_id].instantiate() as TaskActuator
	actuator.initialize(task)
	return actuator
