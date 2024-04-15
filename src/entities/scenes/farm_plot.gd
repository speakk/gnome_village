extends EntityScene

const FARM_PLOT := preload("res://assets/blender_models/farm_plot.blend")

var growth_rate: float = 0.1

var planted_plant: Entity

func _ready() -> void:
	component_container.component_added.connect(func(component: Component) -> void:
		if component is GrowthSpotComponent:
			component.plant_set.connect(func(plant_component: PlantComponent) -> void:
				plant_component.lacks_growth_requirements.connect(self._lacks_growth_requirements)
				plant_component.satisfies_growth_requirements.connect(self._satisfies_growth_requirements)
				)
		)
	super._ready()

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
