extends Node2D

class_name Blueprint

var build_progress := 0.0
var building_type: BuildingTypes.BuildingType

func initialize(_building_type: BuildingTypes.BuildingType) -> Blueprint:
	building_type = _building_type
	modulate = Color(0.2, 0.2, 1.0, 0.2)
	return self

func increase_build_progress(amount: float) -> void:
	build_progress += amount
	if is_finished():
		Events.blueprint_finished.emit(self)
		modulate = Color.WHITE

func is_finished() -> bool:
	return build_progress >= 1.0

func has_materials() -> bool:
	var material_requirements := BuildingTypes.get_building_requirements(building_type)
	
	for deposited_item in $Inventory.get_items() as Array[Item]:
		pass
	
	return true
