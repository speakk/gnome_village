class_name BlueprintTree extends Task

var blueprint: Entity:
	set(_blueprint):
		blueprint = _blueprint

func _init(tile_target: Variant = null, _blueprint: Entity = null) -> void:
	task_name = "Construct blueprint"
	name = "BlueprintTree"
	order_type = Task.OrderType.Sequence
	blueprint = _blueprint
	
	if not _blueprint:
		return
	
	var item := blueprint.definition
	var constructable_component: ConstructableComponent = _blueprint.component_container.get_by_id(Components.Id.Constructable)
	var material_requirements := constructable_component.requirements
	
	if material_requirements.size() > 0:
		var bring_resources := Task.new()
		bring_resources.order_type = Task.OrderType.Parallel
		bring_resources.name = "Bring_Resources"
		bring_resources.task_name = "[parallel]"
		
		# TODO: Each amount gets split into 1
		# Figure if we want to support item stacks being delivered
		for material_requirement in material_requirements as Array[ItemRequirement]:
			for i in material_requirement.amount:
				var requirement := ItemRequirement.new()
				requirement.item = material_requirement.item
				requirement.amount = 1
				var task := BringResourceTask.new({
					item_requirement = requirement,
					inventory_component = constructable_component.get_inventory()
				})
				bring_resources.register_subtask(task)
		
		register_subtask(bring_resources)
	
	var build_task := BuildTask.new({
		constructable_component = constructable_component
	})
	register_subtask(build_task)

func _ready() -> void:
	Events.blueprint_cancel_issued.connect(func(_blueprint: Entity) -> void:
		if _blueprint == blueprint:
			is_cancelled = true
	)

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["blueprint_id"] = SaveSystem.get_save_id(blueprint)
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(dict["blueprint_id"], func(new_blueprint: Entity) -> void:
		blueprint = new_blueprint
		))
#endregion
