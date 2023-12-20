extends Resource

class_name CraftingMaterials

enum CraftingMaterialId {
	Wood, Stone
}

var list: Dictionary = {
	CraftingMaterialId.Wood: preload("res://crafting_materials/wood.tres"),
	CraftingMaterialId.Stone: preload("res://crafting_materials/stone.tres"),
}

func get_by_id(id: CraftingMaterialId) -> CraftingMaterial:
	return list[id]
