extends Node

enum TaskId {
	BringResource, Build, Dismantle
}

var task_actuators: Dictionary = {
	TaskId.BringResource: preload("res://src/tasks/task_actuators/bring_resource.tscn"),
	TaskId.Build: preload("res://src/tasks/task_actuators/build.tscn"),
	TaskId.Dismantle: preload("res://src/tasks/task_actuators/dismantle.tscn"),
}

func create_task_actuator(task: Task) -> TaskActuator:
	var actuator := task_actuators[task.task_id].instantiate() as TaskActuator
	actuator.initialize(task)
	return actuator
