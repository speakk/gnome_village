extends Node

enum TaskId {
	BringResource, Build, Dismantle
}

var task_actuators: Dictionary = {
	TaskId.BringResource: preload("res://tasks/bring_resource_task.tscn"),
	TaskId.Build: preload("res://tasks/build_task.tscn"),
	TaskId.Dismantle: preload("res://tasks/dismantle_task.tscn"),
}

func create_task_actuator(task_id: TaskId, params: Dictionary = {}) -> Task:
	var actuator := task_actuators[task_id].instantiate() as Task
	return actuator
