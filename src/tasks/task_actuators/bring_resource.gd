extends TaskActuator

class_name BringResourceActuator

var _item_amount_component: ItemAmountComponent

# TODO: Handle amounts
func find_closest_material(_item_requirement: ItemRequirement) -> ItemOnGround:
	var materials_on_ground := get_tree().get_nodes_in_group("item_on_ground") as Array[Node]
	var correct_materials := materials_on_ground.filter(func(material: ItemOnGround) -> bool:
		return (material.item_id == _item_requirement.item_id and not material.reserved_for_picking) \
		or material.inventory.has_item_requirement(_item_requirement)
	)
	
	var closest_distance := 99999999.0
	var closest_material: ItemOnGround
	for material_on_ground in correct_materials as Array[ItemOnGround]:
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
		
		if material.item_id == task.item_requirement.item_id:
			_item_amount_component = material.item_amount
		else:
			_item_amount_component = material.inventory.get_item_amount(task.item_requirement.item_id)
	
		var reservation := ItemAmountReservation.new(tree.actor, task.item_requirement.amount)
		_item_amount_component.add_reservation(reservation)
		
	%GoToResource.target_coordinate = Globals.get_map().global_position_to_coordinate(_item_amount_component.get_owner().global_position)
	
	%GetItemFromGround.item_amount_component = _item_amount_component
	%GetItemFromGround.item_requirement = task.item_requirement
	
	if task.target_coordinate:
		%GoToBlueprint.target_coordinate = task.target_coordinate
	else:
		%GoToBlueprint.target_coordinate = Globals.get_map().global_position_to_coordinate(task.inventory.get_owner().global_position)
	
	
	# TODO: Support for just placing items down instead of adding to inventory
	%PutItemToBlueprint.target_inventory = task.inventory
	%PutItemToBlueprint.item_id = task.item_requirement.item_id
	%PutItemToBlueprint.amount = task.item_requirement.amount
	
	%HasItemRequirement.item_requirement = task.item_requirement

func _ready() -> void:
	super._ready()

func clean_up() -> void:
	if _item_amount_component:
		var reservation := ItemAmountReservation.new(tree.actor, task.item_requirement.amount)
		_item_amount_component.remove_reservation(reservation)

func save() -> Dictionary:
	var save_dict := super.save()
	if _item_amount_component:
		save_dict["_item_amount_component_id"] = SaveSystem.save_entity(_item_amount_component)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	
	if save_dict.has("_item_amount_component_id"):
		_item_amount_component = SaveSystem.get_saved_entity(save_dict["_item_amount_component_id"])
