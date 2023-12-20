extends Node

class_name MaterialRequirement

var material_type: CraftingMaterials.CraftingMaterialId
var amount: int

func _init(new_material_type: CraftingMaterials.CraftingMaterialId = CraftingMaterials.CraftingMaterialId.Wood, new_amount: int = 1) -> void:
	material_type = new_material_type
	amount = new_amount
