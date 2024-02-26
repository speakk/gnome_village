class_name PlantedPlant extends Node3D

@onready var component_container: ComponentContainer = $ComponentContainer

signal matured

var current_growth_timer: float = 0.0
var current_growth_stage_index: int = -1

var current_growth_scene: Variant

signal satisfies_growth_requirements
signal lacks_growth_requirements

var plant: Plant
var grows_in: GrowthSpotComponent

func set_plant_id(plant_id: Plants.Id) -> void:
	plant = Plants.get_plant_by_id(plant_id)
	component_container.get_by_id(Components.Id.DisplayName).display_name = plant.display_name

func is_mature() -> bool:
	return current_growth_stage_index >= plant.growth_stages.size() - 1

func has_growth_requirements() -> bool:
	if not grows_in:
		return false
		
	for growth_requirement in plant.growth_requirements:
		var satisfies_requirement := false
		for growth_provided: ItemAmountComponent in grows_in.growth_requirement_inventory.get_items():
			if growth_provided.id == growth_requirement.item_id \
			and growth_provided.amount >= growth_requirement.amount:
				satisfies_requirement = true
				break
		
		if not satisfies_requirement:
			return false
	
	return true

func consume_growth_requirements() -> void:
	for growth_requirement in plant.growth_requirements:
		grows_in.consume_growth_requirement(growth_requirement.item_id, growth_requirement.amount)

func advance_growth_stage() -> void:
	if not is_mature():
		current_growth_stage_index += 1
		
		if current_growth_scene:
			current_growth_scene.queue_free()
		
		var growth_stage_scene := plant.growth_stages[current_growth_stage_index].mesh_scene.instantiate()
		current_growth_scene = growth_stage_scene
		add_child(current_growth_scene)
		
		if is_mature():
			$ParticlesMatured.emitting = true
			matured.emit()

var lacks_growth_requirements_emitted := false

func _physics_process(delta: float) -> void:
	if not plant:
		return
	
	if not is_mature():
		if has_growth_requirements():
			lacks_growth_requirements_emitted = false
			satisfies_growth_requirements.emit()
			current_growth_timer += delta
			if current_growth_timer > plant.growth_stage_length:
				advance_growth_stage()
				consume_growth_requirements()
				current_growth_timer = 0
		elif not lacks_growth_requirements_emitted:
			lacks_growth_requirements_emitted = true
			lacks_growth_requirements.emit()

static func create_from_id(plant_id: Plants.Id) -> PlantedPlant:
	var PLANTED_PLANT := load("res://src/plants/PlantedPlant.tscn") as PackedScene
	var planted_plant := PLANTED_PLANT.instantiate()
	planted_plant.set_plant_id(plant_id)
	return planted_plant
