class_name PlantSceneComponent extends SceneComponent

var SCENE := load("res://src/entities/scenes/Plant.tscn")

func _init() -> void:
	super._init()
	id = Components.Id.Scene
	scene = SCENE
	subscriptions.append(
		Subscription.new(id, Components.Id.Plant, func(plant_component: PlantComponent) -> void:
			(get_owner() as PlantScene).set_plant(plant_component)
			)
	)
