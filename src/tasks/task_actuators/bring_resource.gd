class_name BringResourceActuator extends TaskActuator

var _item_amount_component: ItemAmountComponent

# TODO: Handle amounts
func find_closest_material(_item_requirement: ItemRequirement) -> Entity:
	var materials_on_ground := get_tree().get_nodes_in_group("entity") as Array[Node]
	
	# TODO: Doesn't handle reservations yet with material.item_amount
	var correct_materials := materials_on_ground.filter(func(material: Entity) -> bool:
		var container: ComponentContainer = material.component_container
		
		var constructable: ConstructableComponent = container.get_by_id(Components.Id.Constructable)
		if constructable and not constructable.is_finished:
			return false
		
		if material.definition and material.definition == _item_requirement.item:
			if container.has_component(Components.Id.ItemAmount):
				var item_amount: ItemAmountComponent = container.get_by_id(Components.Id.ItemAmount)
				if item_amount.has_item_amount(_item_requirement.item, _item_requirement.amount):
					
					# Filter out items that were already marked unreachable
					var actor_position: Vector3 = tree.actor.global_position
					var from: Vector2i = Globals.get_map().global_position_to_coordinate(actor_position)
					var to: Vector2i = Globals.get_map().global_position_to_coordinate(material.global_position)
					var path_invalid := PathFinder.is_path_marked_unreachable(from, to)
					
					if not path_invalid:
						return true
			
		if container.has_component(Components.Id.Inventory):
			var inventory: InventoryComponent = container.get_by_id(Components.Id.Inventory)
			if inventory.items_can_be_picked and inventory.has_item_requirement(_item_requirement):
				return true
		
		return false
	)
	
	var closest_distance := 99999999.0
	var closest_material: Entity
	for material_on_ground in correct_materials as Array[Entity]:
		var distance: float = tree.actor.global_position.distance_to(material_on_ground.global_position)
		if distance < closest_distance:
			closest_distance = distance
			closest_material = material_on_ground
	
	return closest_material

func initialize(_task: BringResourceTask) -> BringResourceActuator:
	task = _task
	return self

func start_work() -> void:
	super.start_work()
	if not _item_amount_component:
		var material := find_closest_material(task.item_requirement)
		if not material:
			task.has_failed = true
			return
		
		if material.definition and material.definition == task.item_requirement.item:
			_item_amount_component = material.component_container.get_by_id(Components.Id.ItemAmount)
		else:
			_item_amount_component = material.component_container.get_by_id(Components.Id.Inventory).get_item_amount(task.item_requirement.item)
	
		var reservation := ItemAmountReservation.new(tree.actor, task.item_requirement.amount)
		_item_amount_component.add_reservation(reservation)
		
	%GoToResource.target_coordinate = Globals.get_map().global_position_to_coordinate(_item_amount_component.get_owner().global_position)
	
	%GetItemFromGround.item_amount_component = _item_amount_component
	%GetItemFromGround.item_requirement = task.item_requirement
	
	if task.target_coordinate:
		%GoToBlueprint.target_coordinate = task.target_coordinate
	else:
		if not is_instance_valid(task.inventory_component.get_owner()):
			task.has_failed = true
			return
			
		%GoToBlueprint.target_coordinate = Globals.get_map().global_position_to_coordinate(task.inventory_component.get_owner().global_position)
	
	
	# TODO: Support for just placing items down instead of adding to inventory
	%PutItemToBlueprint.target_inventory = task.inventory_component
	%PutItemToBlueprint.item = task.item_requirement.item
	%PutItemToBlueprint.amount = task.item_requirement.amount
	
	# TODO: This is too much manual work, make FailTask
	# somehow just fail things automatically
	%GoToBlueprint.failed.connect(func() -> void:
		push_warning("gotoblueprint failed")
		fail()
		)
	%GoToResource.failed.connect(func() -> void:
		push_warning("gotoresource failed")
		fail()
		)
	%FailTask.failed.connect(func() -> void:
		push_warning("failtask failed lol")
		fail()
		)
	
	if not task.item_requirement:
		print("Oopsie")
	%HasItemRequirement.item_requirement = task.item_requirement

func _ready() -> void:
	super._ready()

func clean_up() -> void:
	if _item_amount_component:
		var reservation := ItemAmountReservation.new(tree.actor, task.item_requirement.amount)
		_item_amount_component.remove_reservation(reservation)
