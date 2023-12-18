extends BeehaveTree

class_name BluePrintTaskAssigner

var TASK_ASSIGNER := preload("res://task_assigners/TaskAssigner.tscn")

var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

func initialize(tile_target: Vector2i, blueprint: Blueprint) -> BluePrintTaskAssigner:
	%FinishAssigner.assigner = self
	var building_type := blueprint.building_type
	var material_requirements := BuildingTypes.get_building_requirements(building_type)
	for material_requirement in material_requirements:
		var bring_resource_task := (BRING_RESOURCE_TASK.instantiate() as BringResourceTask).initialize(tile_target, material_requirement)
		var bring_material_task_assigner := (TASK_ASSIGNER.instantiate() as TaskAssigner).initialize(bring_resource_task)
		%BringResources.add_child(bring_material_task_assigner)

	var build_task := (BUILD_TASK.instantiate() as BuildTask).initialize(blueprint)
	%BuildAssigner.initialize(build_task)
	
	return self
	#blackboard.set_value()
