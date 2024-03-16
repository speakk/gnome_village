class_name SleepTask extends Task

func _init() -> void:
	task_id = Tasks.TaskId.Sleep
	task_name = "Sleep"
	animation_name = "Sleep"

func create_action(actor: Settler) -> ActorAction:
	return SleepActorAction.new(actor, self)

func get_target(actor: Settler) -> Vector3:
	return actor.global_position
