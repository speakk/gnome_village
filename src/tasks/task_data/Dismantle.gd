class_name DismantleTask extends Task

var target: Entity

func _init(params: Variant = null) -> void:
	task_id = Tasks.TaskId.Dismantle
	task_name = "Dismantle"
	animation_name = "Build"
	
	if not params is Dictionary: return
	
	target = params["target"]
	#target.delete_called.connect(func() -> void:
		#cancelled.emit()
		#)
	target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true

func _ready() -> void:
	Events.dismantle_cancel_issued.connect(func(_entity: Entity) -> void:
		if _entity == target:
			is_cancelled = true
	)

func create_action(actor: Settler) -> ActorAction:
	var action: DismantleActorAction = DismantleActorAction.new(actor, self)
	action.finished.connect(func() -> void:
		is_finished = true
		)
	return action

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
