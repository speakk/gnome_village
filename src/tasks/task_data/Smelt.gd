class_name SmeltTask extends Task

var smelter_component: SmelterComponent

func _init(_smelter_component: SmelterComponent = null) -> void:
	task_id = Tasks.TaskId.Smelt
	task_name = "Smelt"
	animation_name = "Build"
	
	smelter_component = _smelter_component
	smelter_component.removed.connect(func() -> void:
		is_cancelled = true
		)

func create_action(actor: Settler) -> ActorAction:
	return SmeltActorAction.new(actor, self)

func get_target(actor: Settler) -> Vector3:
	return smelter_component.position

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["smelter_owner_id"] = SaveSystem.get_save_id(smelter_component.get_owner())
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["smelter_owner_id"], func(comp_owner: Entity) -> void:
		smelter_component = comp_owner.component_container.get_by_id(Components.Id.Smelter)
		))
#endregion

