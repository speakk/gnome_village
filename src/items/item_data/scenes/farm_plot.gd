extends Node3D

var growth_rate: float = 0.1

var planted_plant: PlantedPlant

func _physics_process(delta: float) -> void:
	pass

func start_growing_plant(plant_id: Plants.Id) -> void:
	if not planted_plant:
		planted_plant = PlantedPlant.create_from_id(plant_id)
		add_child(planted_plant)
