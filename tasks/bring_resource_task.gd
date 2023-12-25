extends Task

class_name BringResourceTask

var target_tile: Vector2i
var item_requirement: ItemRequirement 
var blueprint: Blueprint

var _material: ItemOnGround

# TODO: Handle amounts
func find_closest_material(_item_requirement: ItemRequirement, tree: SceneTree) -> ItemOnGround:
	var materials_on_ground := tree.get_nodes_in_group("item_on_ground") as Array[Node]
	var correct_materials := materials_on_ground.filter(func(material: ItemOnGround) -> bool:
		return material.item_id == _item_requirement.item_id and not material.reserved_for_picking
	)
	
	var closest_distance := 99999999.0
	var closest_material: ItemOnGround
	for material_on_ground in correct_materials as Array[ItemOnGround]:
		var distance: float = actor.global_position.distance_to(material_on_ground.global_position)
		if distance < closest_distance:
			closest_distance = distance
			closest_material = material_on_ground
	
	return closest_material

func initialize(_target_tile: Vector2i, _item_requirement: ItemRequirement, _blueprint: Blueprint, tree: SceneTree) -> BringResourceTask:
	target_tile = _target_tile
	item_requirement = _item_requirement
	blueprint = _blueprint

	return self

func _ready() -> void:
	var material := find_closest_material(item_requirement, get_tree())
	
	if not material:
		return
	
	_material = material
	
	material.reserved_for_picking = true
	#%GoToResource.target = (Globals.get_map() as MainMap).map_to_local(material.global_position)
	%GoToResource.target = material.global_position
	
	%GetItemFromGround.target_item = material
	%GetItemFromGround.amount = item_requirement.amount
	
	%GoToBlueprint.target = (Globals.get_map() as MainMap).map_to_local(target_tile)
	
	%PutItemToBlueprint.target_inventory = blueprint.get_node("Inventory")
	%PutItemToBlueprint.item_id = item_requirement.item_id
	%PutItemToBlueprint.amount = item_requirement.amount
	

func tick() -> int:
	return super.tick()
