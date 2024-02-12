class_name PlantedPlant extends Node3D


signal matured

var current_growth_timer: float = 0.0
var current_growth_stage_index: int = -1

var current_growth_scene: Variant

var plant: Plant

func set_plant_id(plant_id: Plants.Id) -> void:
	plant = Plants.get_plant_by_id(plant_id)

func is_mature() -> bool:
	return current_growth_stage_index >= plant.growth_stages.size() - 1

func has_growth_requirements() -> bool:
	return true

func advance_growth_stage() -> void:
	if not is_mature():
		current_growth_stage_index += 1
		
		if current_growth_scene:
			current_growth_scene.queue_free()
		
		var growth_stage_scene := plant.growth_stages[current_growth_stage_index].mesh_scene.instantiate()
		current_growth_scene = growth_stage_scene
		add_child(current_growth_scene)
		
		if is_mature():
			matured.emit()
	
func _physics_process(delta: float) -> void:
	if not plant:
		return
		
	if has_growth_requirements():
		current_growth_timer += delta
		if current_growth_timer > plant.growth_stage_length:
			advance_growth_stage()
			current_growth_timer = 0

static func create_from_id(plant_id: Plants.Id) -> PlantedPlant:
	var PLANTED_PLANT := preload("res://src/plants/PlantedPlant.tscn")
	var planted_plant := PLANTED_PLANT.instantiate()
	planted_plant.set_plant_id(plant_id)
	return planted_plant
