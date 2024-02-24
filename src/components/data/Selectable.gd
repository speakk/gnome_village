class_name SelectableComponent extends Component

func _init() -> void:
	id = Components.Id.Selectable
	instance = preload("res://src/components/instances/Selectable.gd")
