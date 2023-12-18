extends Task

class_name BringResourceTask

var target_tile: Vector2i
var material_requirement: Materials.MaterialRequirement 

func initialize(_target_tile: Vector2i, _material_requirement: Materials.MaterialRequirement) -> BringResourceTask:
	target_tile = _target_tile
	material_requirement = _material_requirement
	# TODO: Just uhhh, we need a node2d for the target, right now we're just passing target_tile.
	# Should GoTo and Bring Resource target a targe tile or just straight up a node?
	# There should probably be a real node where the blueprint is, shouldn't there?
	%FetchResource.target = (Globals.get_map() as MainMap).map_to_local(target_tile)
	return self

func tick() -> int:
	return super.tick()
