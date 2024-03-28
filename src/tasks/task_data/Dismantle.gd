class_name DismantleTask extends Task

var target: Entity

func _init(params: Dictionary) -> void:
	task_id = Tasks.TaskId.Dismantle
	task_name = "Dismantle"
	animation_name = "Build"
	
	target = params["target"]
	target.component_container.get_by_id(Components.Id.Constructable).reserved_for_dismantling = true

func _ready() -> void:
	Events.dismantle_cancel_issued.connect(func(_entity: Entity) -> void:
		if _entity == target:
			is_cancelled = true
	)
#
#func save() -> Dictionary:
	#var save_dict: Dictionary = super.save()
	#save_dict["target_id"] = SaveSystem.save_entity(target)
	#return save_dict
#
#func load_save(save_dict: Dictionary) -> void:
	#super.load_save(save_dict)
	#target = SaveSystem.get_saved_entity(save_dict["target_id"])

func create_action(actor: Settler) -> ActorAction:
	return DismantleActorAction.new(actor, self)

func get_target(actor: Settler) -> Vector3:
	return target.global_position
