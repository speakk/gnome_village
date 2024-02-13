extends Node3D

const FARM_PLOT := preload("res://assets/blender_models/farm_plot.blend")

var growth_rate: float = 0.1

var planted_plant: PlantedPlant

func _physics_process(delta: float) -> void:
	pass

func start_growing_plant(plant_id: Plants.Id) -> void:
	if not planted_plant:
		planted_plant = PlantedPlant.create_from_id(plant_id)
		add_child(planted_plant)

func set_as_blueprint(is_blueprint: bool) -> void:
	if is_blueprint:
		Globals.apply_blueprint_material($farm_plot)
	else:
		$farm_plot.queue_free()
		var new_scene := FARM_PLOT.instantiate()
		new_scene.name = "farm_plot"
		add_child(new_scene)
