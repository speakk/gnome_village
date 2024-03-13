class_name EatTask extends Task

var consumable: ConsumableComponent

func _init(params: Dictionary) -> void:
	task_id = Tasks.TaskId.Eat
	task_name = "Eat food"
	
	consumable = params["consumable"]
	#target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true
