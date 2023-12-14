extends Node

enum BuildingType {
	Wall
}

var building_requirements := {
	BuildingType.Wall: [Materials.MaterialRequirement.new(Materials.MaterialTypes.Wood, 5)]
}

func get_building_requirements(building_type: BuildingType) -> Array[Materials.MaterialRequirement]:
	return building_requirements.get(building_requirements) as Array[Materials.MaterialRequirement]
