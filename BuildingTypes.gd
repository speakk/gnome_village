extends Node

enum BuildingType {
	Wall
}

var building_requirements := {
	BuildingType.Wall: [MaterialRequirement.new(CraftingMaterials.CraftingMaterialId.Wood, 5)] as Array[MaterialRequirement]
}

func get_building_requirements(building_type: BuildingType) -> Array[MaterialRequirement]:
	return building_requirements[building_type] as Array[MaterialRequirement]
