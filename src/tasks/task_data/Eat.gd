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

func get_target(actor: Settler) -> Vector3:
	return consumable.get_owner().global_position

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["consumable_owner_id"] = SaveSystem.get_save_id(consumable.get_owner())
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["consumable_owner_id"], func(comp_owner: Entity) -> void:
		consumable = comp_owner.component_container.get_by_id(Components.Id.Consumable)
		))
#endregion
