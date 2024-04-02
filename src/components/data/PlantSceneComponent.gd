class_name PlantSceneComponent extends SceneComponent

var SCENE := load("res://src/entities/scenes/Plant.tscn")

func _init() -> void:
	super._init()
	id = Components.Id.Scene
	scene = SCENE
