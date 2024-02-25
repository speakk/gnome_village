class_name Component extends Resource

@export var id: Components.Id

var component_owner: Node3D

func set_owner(_new_owner: Node3D) -> void:
	component_owner = _new_owner
