class_name WorldPositionComponent extends Component

func _init() -> void:
	id = Components.Id.WorldPosition
	instance = preload("res://src/components/instances/WorldPosition.gd")
