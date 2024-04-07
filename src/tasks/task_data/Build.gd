class_name BuildTask extends Task

var constructable_component: ConstructableComponent

func _init(params: Variant = null) -> void:
	task_id = Tasks.TaskId.Build
	task_name = "Build"
	animation_name = "Build"
	
	if not params is Dictionary: return
	
	constructable_component = params["constructable_component"]
	constructable_component.removed.connect(func() -> void:
		is_cancelled = true
		)
#
#func save() -> Dictionary:
	#var save_dict: Dictionary = super.save()
	#save_dict["constructable_component_id"] = SaveSystem.save_entity(constructable_component)
	#return save_dict
#
#func load_save(save_dict: Dictionary) -> void:
	#super.load_save(save_dict)
	#constructable_component = SaveSystem.get_saved_entity(save_dict["constructable_component_id"])

func create_action(actor: Settler) -> ActorAction:
	return BuildActorAction.new(actor, self)

func get_target(actor: Settler) -> Vector3:
	return constructable_component.get_owner().global_position

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["constructable_owner_id"] = SaveSystem.get_save_id(constructable_component.get_owner())
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["constructable_owner_id"], func(comp_owner: Entity) -> void:
		constructable_component = comp_owner.component_container.get_by_id(Components.Id.Constructable)
		))
#endregion
