class_name EatTask extends Task

var consumable: ConsumableComponent

func _init(params: Dictionary) -> void:
	task_id = Tasks.TaskId.Eat
	task_name = "Eat food"
	animation_name = "Eat"
	
	consumable = params["consumable"]
	consumable.reserved = true
	#target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true

func create_action(actor: Settler) -> ActorAction:
	return EatActorAction.new(actor, self)

func get_target() -> Vector3:
	return consumable.get_owner().global_position
