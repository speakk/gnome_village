class_name EatTask extends Task

var consumable: ConsumableComponent

func _init(params: Dictionary) -> void:
	task_id = Tasks.TaskId.Eat
	task_name = "Eat food"
	task_actuator_scene = preload("res://src/tasks/task_actuators/eat.tscn")
	
	consumable = params["consumable"]
	consumable.reserved = true
	#target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true
