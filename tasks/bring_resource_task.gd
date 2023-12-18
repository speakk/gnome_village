extends Task

class_name BringResourceTask

# TODO:
# - Find material (if fail then FAILURE)
# - GoTo Material
# - PickUp Material
# - GoTo target_tile
# - Deposit Material (not sure where to, yet... the blueprint itself, which is not a node yet)

var target_tile: Vector2i
var material_requirement: Materials.MaterialRequirement 

func initialize(_target_tile: Vector2i, _material_requirement: Materials.MaterialRequirement) -> BringResourceTask:
	target_tile = _target_tile
	material_requirement = _material_requirement

	%FetchResource.target = (Globals.get_map() as MainMap).map_to_local(target_tile)
	# This just a goofy way to have "two" actions right now, follow TODO list and get rid of this
	$SequenceComposite/GoToAction2.target = (Globals.get_map() as MainMap).map_to_local(target_tile)
	return self

func tick() -> int:
	return super.tick()
