class_name FarmPlot extends Node3D

@onready var growth_spot: GrowthSpotComponent = get_parent().component_container.get_by_id(Components.Id.GrowthSpot)

const FARM_PLOT := preload("res://assets/blender_models/farm_plot.blend")

var growth_rate: float = 0.1

var planted_plant: ItemOnGround
var plant_component: PlantComponent
@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

func _physics_process(delta: float) -> void:
	pass

func _ready() -> void:
	# TODO: Test add some water to growth spot
	growth_spot.increase_growth_requirement(Items.Id.Water, 10)
	growth_spot.plant_id_set.connect(_plant_id_set)

func _plant_id_set(plant_id: Items.Id) -> void:
	if not planted_plant:
		var planted_plant: ItemOnGround = ITEM_ON_GROUND.instantiate()
		add_child(planted_plant)
		planted_plant.initialize(plant_id)
		plant_component = planted_plant.component_container.get_by_id(Components.Id.Plant)
		plant_component.grows_in = growth_spot
		plant_component.lacks_growth_requirements.connect(_lacks_growth_requirements)
		plant_component.satisfies_growth_requirements.connect(_satisfies_growth_requirements)

func _lacks_growth_requirements() -> void:
	$LacksGrowthRequirementIndicator.show()
	Events.farm_plot_plant_lacks_growth_requirement.emit(self.get_parent())

func _satisfies_growth_requirements() -> void:
	$LacksGrowthRequirementIndicator.hide()

func set_blueprint(is_blueprint: bool) -> void:
	if is_blueprint:
		Globals.apply_blueprint_material($farm_plot)
	else:
		if has_node("farm_plot"):
			$farm_plot.queue_free()
			var new_scene := FARM_PLOT.instantiate()
			new_scene.name = "farm_plot"
			add_child(new_scene)
