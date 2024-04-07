class_name DismantleTask extends Task

var target: Entity

func _init(params: Variant = null) -> void:
	task_id = Tasks.TaskId.Dismantle
	task_name = "Dismantle"
	animation_name = "Build"
	
	if not params is Dictionary: return
	
	target = params["target"]
	target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true

func _ready() -> void:
	Events.dismantle_cancel_issued.connect(func(_entity: Entity) -> void:
		if _entity == target:
			is_cancelled = true
	)


# TODO: Think about how to do this:
# When we save and dismantle has destroyed something
# How do we cancel those tasks?

# Current thought: The tasks themselves maybe connect
# to signals of their targets to check if on_exit has been
# called to the targets, the tasks either fail or get
# automatically cancelled

#func cancels_tasks(tasks: Array[Task]) -> Array[Task]:
	#var cancelled = tasks.filter(func(task: Task) -> bool:
		##if task.task_id == Tasks.TaskId.BringResource
		#if task.has()
		#)

func create_action(actor: Settler) -> ActorAction:
	return DismantleActorAction.new(actor, self)

func get_target(actor: Settler) -> Vector3:
	return target.global_position

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["target_id"] = SaveSystem.get_save_id(target)
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(
		SaveSystem.EntityReferenceEntry.new(dict["target_id"], func(new_target: Entity) -> void:
		target = new_target
		))
#endregion
