extends BeehaveTree

var TASK_ASSIGNER := preload("res://TaskAssigner.tscn")
var BRING_RESOURCE_TASK := preload("res://tasks/bring_resource_task.tscn")
var BUILD_TASK := preload("res://tasks/build_task.tscn")

@onready var build: TaskAssigner = $"%Build"

func initialize(tile_target: Vector2i, building_type: BuildingTypes.BuildingType) -> void:
	var material_requirements := BuildingTypes.get_building_requirements(building_type)
	for material_requirement in material_requirements:
		var bring_material_task_assigner := TASK_ASSIGNER.instantiate() as TaskAssigner
		bring_material_task_assigner.task = (BRING_RESOURCE_TASK.instantiate() as BringResourceTask).initialize(tile_target, material_requirement)
		$BringResources.add_child(bring_material_task_assigner)

	var build_task := (BUILD_TASK.instantiate() as BuildTask).initialize(tile_target)
	build.task = build_task

	#blackboard.set_value()
