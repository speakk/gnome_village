extends Task

class_name BringResourceTask

var target_tile: Vector2i
var item_requirement: ItemRequirement 
var blueprint: ItemOnGround

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

func initialize(_target_tile: Vector2i, _item_requirement: ItemRequirement, _blueprint: ItemOnGround) -> BringResourceTask:
	target_tile = _target_tile
	item_requirement = _item_requirement
	blueprint = _blueprint

	%GoToBlueprint.target_coordinate = target_tile
	
	%PutItemToBlueprint.target_inventory = blueprint.get_node("ConstructionInventory")
	%PutItemToBlueprint.item_id = item_requirement.item_id
	%PutItemToBlueprint.amount = item_requirement.amount

	return self

func start_work() -> void:
	super.start_work()
	var material := find_closest_material(item_requirement)
	
	if not material:
		return
	
	_material = material
	
	material.reserved_for_picking = true
	
	%GoToResource.target_coordinate = Globals.get_map().global_position_to_coordinate(material.global_position)
	
	%GetItemFromGround.target_item = material
	%GetItemFromGround.amount = item_requirement.amount

func _ready() -> void:
	super._ready()

func clean_up() -> void:
	if _material:
		_material.reserved_for_picking = false

func save() -> Dictionary:
	var save_dict := super.save()
	save_dict["target_tile.x"] = target_tile.x
	save_dict["target_tile.y"] = target_tile.y
	save_dict["item_requirement_id"] = SaveSystem.save_entity(item_requirement)
	
	if blueprint:
		save_dict["blueprint_save_id"] = SaveSystem.save_entity(blueprint)
	
	if _material:
		save_dict["_material_save_id"] = SaveSystem.save_entity(_material)
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	super.load_save(save_dict)
	target_tile = Vector2i(save_dict["target_tile.x"], save_dict["target_tile.y"])
	#item_requirement = ItemRequirement.new()
	#item_requirement.load_save(save_dict["item_requirement"])
	
	blueprint = SaveSystem.get_saved_entity(save_dict["blueprint_save_id"])
	item_requirement = SaveSystem.get_saved_entity(save_dict["item_requirement_id"])
	
	if save_dict.has("_material_save_id"):
		_material = SaveSystem.get_saved_entity(save_dict["_material_save_id"])
	
	#if save_dict.has("item_requirement_id"):
		#SaveSystem.register_load_reference(self, "item_requirement", save_dict["item_requirement_id"])
	##if save_dict.has("blueprint_save_id"):
	##	SaveSystem.register_load_reference(self, "blueprint", save_dict["blueprint_save_id"])
	#if save_dict.has("_material_save_id"):
		#SaveSystem.register_load_reference(self, "_material", save_dict["_material_save_id"])
