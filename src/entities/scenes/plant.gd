class_name PlantScene extends EntityScene

var current_growth_scene: Node3D

var plant: PlantComponent

func _ready() -> void:
	var plant_component: PlantComponent = component_container.get_by_id(Components.Id.Plant)
	if plant_component:
		plant = plant_component
		plant_component.advanced_growth_stage.connect(_advance_growth_stage)
		plant_component.matured.connect(_matured)
		
	component_container.component_added.connect(func(component: Component) -> void:
		if component is PlantComponent:
			plant = component
			plant.advanced_growth_stage.connect(_advance_growth_stage)
			plant.matured.connect(_matured)
		)
	
	super._ready()
	name = "PlantScene"

func _matured() -> void:
	$ParticlesMatured.emitting = true

func _advance_growth_stage(new_index: int) -> void:
	if not plant.is_mature():
		if current_growth_scene:
			current_growth_scene.queue_free()
		
		var growth_stage_scene := plant.growth_stages[new_index].mesh_scene.instantiate()
		current_growth_scene = growth_stage_scene
		add_child(current_growth_scene)
		
