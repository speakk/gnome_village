extends TaskActuator

class_name BringResourceActuator

var _material: ItemOnGround

# TODO: Handle amounts
func find_closest_material(_item_requirement: ItemRequirement) -> ItemOnGround:
	var materials_on_ground := get_tree().get_nodes_in_group("item_on_ground") as Array[Node]
	var correct_materials := materials_on_ground.filter(func(material: ItemOnGround) -> bool:
		return material.item_id == _item_requirement.item_id and not material.reserved_for_picking
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
	if not _material:
		var material := find_closest_material(task.item_requirement)
		if not material:
			print("No material, fail")
			task.has_failed = true
			return
	
		_material = material
	
	_material.reserved_for_picking = true
	
	%GoToResource.target_coordinate = Globals.get_map().global_position_to_coordinate(_material.global_position)
	
	%GetItemFromGround.target_item = _material
	%GetItemFromGround.amount = task.item_requirement.amount
	
	%GoToBlueprint.target_coordinate = task.target_coordinate
	
	%PutItemToBlueprint.target_inventory = task.inventory_holder_entity.get_node("ConstructionInventory")
	%PutItemToBlueprint.item_id = task.item_requirement.item_id
	%PutItemToBlueprint.amount = task.item_requirement.amount
	
	%HasItemRequirement.item_requirement = task.item_requirement

func _ready() -> void:
	super._ready()

func clean_up() -> void:
	if _material:
		_material.reserved_for_picking = false

func save() -> Dictionary:
	var save_dict := super.save()
	if _material:
		save_dict["_material_save_id"] = SaveSystem.save_entity(_material)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	
	if save_dict.has("_material_save_id"):
		_material = SaveSystem.get_saved_entity(save_dict["_material_save_id"])
