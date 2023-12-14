extends Node

enum MaterialTypes {
	Wood, Stone
}

class MaterialRequirement:
	var material_type: MaterialTypes
	var amount: int
	
	func _init(new_material_type: MaterialTypes = MaterialTypes.Wood, new_amount: int = 1) -> void:
		material_type = new_material_type
		amount = new_amount
