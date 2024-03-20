class_name FarmPlot extends Node3D

#@onready var growth_spot: GrowthSpotComponent = get_parent().component_container.get_by_id(Components.Id.GrowthSpot)
@onready var growth_spot: GrowthSpotComponent

const FARM_PLOT := preload("res://assets/blender_models/farm_plot.blend")

var growth_rate: float = 0.1

var planted_plant: ItemOnGround
var plant_component: PlantComponent
@onready var ITEM_ON_GROUND := load("res://src/items/item_on_ground/ItemOnGround.tscn")

func _plant_set(plant: Item) -> void:
	if not planted_plant:
		planted_plant = ITEM_ON_GROUND.instantiate()
		add_child(planted_plant)
		planted_plant.item = plant
		plant_component = planted_plant.component_container.get_by_id(Components.Id.Plant)
		# TODO: This is so that the can be "dismantled". Do this any other way
		# in the future.
		planted_plant.component_container.add_component(ConstructableComponent.new())
		plant_component.grows_in = growth_spot
		plant_component.managed_by_player = true
		plant_component.lacks_growth_requirements.connect(_lacks_growth_requirements)
		plant_component.satisfies_growth_requirements.connect(_satisfies_growth_requirements)

func set_growth_spot(_growth_spot: GrowthSpotComponent) -> void:
	growth_spot = _growth_spot
	growth_spot.plant_set.connect(_plant_set)

func _lacks_growth_requirements() -> void:
	$LacksGrowthRequirementIndicator.show()

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
