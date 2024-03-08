extends Node3D

@onready var component_container: ComponentContainer = $ComponentContainer

var current_growth_scene: Node3D

var plant: PlantComponent

func set_plant(_plant: PlantComponent) -> void:
	plant = _plant
	plant.advanced_growth_stage.connect(_advance_growth_stage)
	plant.matured.connect(_matured)

func _matured() -> void:
	$ParticlesMatured.emitting = true

func _advance_growth_stage(new_index: int) -> void:
	if not plant.is_mature():
		if current_growth_scene:
			current_growth_scene.queue_free()
		
		var growth_stage_scene := plant.growth_stages[new_index].mesh_scene.instantiate()
		print("New growth stage", new_index)
		current_growth_scene = growth_stage_scene
		#Events.request_entity_add.emit(current_growth_scene)
		add_child(current_growth_scene)
		#current_growth_scene.global_position = global_position
		