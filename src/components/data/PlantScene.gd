@tool
class_name PlantSceneComponent extends SceneComponent

var SCENE := load("res://src/entities/definitions/scenes/Plant.tscn")

func _init() -> void:
	id = Components.Id.PlantScene
	scene = SCENE
	subscriptions.append(
		Subscription.new(id, Components.Id.Plant, func(plant_component: PlantComponent) -> void:
			_instantiated_scene.set_plant(plant_component)
			)
	)
