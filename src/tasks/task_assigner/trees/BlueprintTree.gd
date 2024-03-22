class_name BlueprintTree extends Task

var blueprint: Entity

func _init(tile_target: Vector2i, _blueprint: Entity) -> void:
	task_name = "Construct blueprint"
	name = "BlueprintTree"
	order_type = Task.OrderType.Sequence
	blueprint = _blueprint
	
	var item := blueprint.item
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
				task.failed.connect(_handle_task_failure)
				bring_resources.register_subtask(task)
		
		register_subtask(bring_resources)
	
	var build_task := BuildTask.new({
		constructable_component = constructable_component
	})
	build_task.failed.connect(_handle_task_failure)
	register_subtask(build_task)

func _ready() -> void:
	Events.blueprint_cancel_issued.connect(func(_blueprint: Entity) -> void:
		if _blueprint == blueprint:
			is_cancelled = true
	)

func _handle_task_failure() -> void:
	pass
