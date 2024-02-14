extends Node3D

@onready var growth_spot: GrowthSpot = $GrowthSpot

const FARM_PLOT := preload("res://assets/blender_models/farm_plot.blend")

var growth_rate: float = 0.1

var planted_plant: PlantedPlant

func _physics_process(delta: float) -> void:
	pass

func _ready() -> void:
	# TODO: Test add some water to growth spot
	growth_spot.increase_growth_requirement(Items.Id.Water, 10)

func start_growing_plant(plant_id: Plants.Id) -> void:
	if not planted_plant:
		planted_plant = PlantedPlant.create_from_id(plant_id)
		add_child(planted_plant)
		planted_plant.grows_in = growth_spot
		planted_plant.lacks_growth_requirements.connect(_lacks_growth_requirements)
		planted_plant.satisfies_growth_requirements.connect(_satisfies_growth_requirements)

func _lacks_growth_requirements() -> void:
	$LacksGrowthRequirementIndicator.show()

func _satisfies_growth_requirements() -> void:
	$LacksGrowthRequirementIndicator.hide()

func set_as_blueprint(is_blueprint: bool) -> void:
	if is_blueprint:
		Globals.apply_blueprint_material($farm_plot)
	else:
		$farm_plot.queue_free()
		var new_scene := FARM_PLOT.instantiate()
		new_scene.name = "farm_plot"
		add_child(new_scene)
