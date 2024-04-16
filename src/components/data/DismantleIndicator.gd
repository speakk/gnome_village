class_name DismantleIndicatorComponent extends Component

var SCENE := preload("res://src/components/data/scenes/DismantleIndicator.tscn")

func _init() -> void:
	id = Components.Id.DismantleIndicator

func on_enter() -> void:
	var scene := SCENE.instantiate()
	get_owner().add_child(scene)
