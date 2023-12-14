extends Task

class_name BringResourceTask

var target_tile: Vector2i
var material_requirement: Materials.MaterialRequirement 

func initialize(_target_tile: Vector2i, _material_requirement: Materials.MaterialRequirement) -> BringResourceTask:
	target_tile = _target_tile
	material_requirement = _material_requirement
	return self
