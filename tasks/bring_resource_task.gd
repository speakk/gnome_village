extends Task

class_name BringResourceTask

var target_tile: Vector2i
var material_requirement: MaterialRequirement 
var blueprint: Blueprint

var _material: MaterialOnGround

# TODO: Handle amounts
func find_closest_material(_material_requirement: MaterialRequirement, tree: SceneTree) -> MaterialOnGround:
	var materials_on_ground := tree.get_nodes_in_group("material_on_ground") as Array[Node]
	var correct_materials := materials_on_ground.filter(func(material: MaterialOnGround) -> bool:
		return material.material_type == _material_requirement.material_type and not material.reserved_for_picking
	)
	
	var closest_distance := 99999999.0
	var closest_material: MaterialOnGround
	for material_on_ground in correct_materials as Array[MaterialOnGround]:
		var distance: float = actor.global_position.distance_to(material_on_ground.global_position)
		if distance < closest_distance:
			closest_distance = distance
			closest_material = material_on_ground
	
	return closest_material

func initialize(_target_tile: Vector2i, _material_requirement: MaterialRequirement, _blueprint: Blueprint, tree: SceneTree) -> BringResourceTask:
	target_tile = _target_tile
	material_requirement = _material_requirement
	blueprint = _blueprint

	return self

func _ready() -> void:
	var material := find_closest_material(material_requirement, get_tree())
	
	if not material:
		return
	
	_material = material
	
	material.reserved_for_picking = true
	#%GoToResource.target = (Globals.get_map() as MainMap).map_to_local(material.global_position)
	%GoToResource.target = material.global_position
	
	%GetItemFromGround.target_item = material.get_node("Item")
	%GetItemFromGround.amount = material_requirement.amount
	
	%PutItemToBlueprint.target_inventory = blueprint.get_node("Inventory")
	%PutItemToBlueprint.item_id = material_requirement.material_type
	%PutItemToBlueprint.amount = material_requirement.amount
	
	%GoToBlueprint.target = (Globals.get_map() as MainMap).map_to_local(target_tile)

func tick() -> int:
	return super.tick()
