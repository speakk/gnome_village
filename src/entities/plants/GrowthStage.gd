class_name GrowthStage extends Resource

@export var mesh_scene: PackedScene

func serialize() -> Dictionary:
	return {
		mesh_scene_path =  mesh_scene.resource_path
	}

func deserialize(dict: Dictionary) -> void:
	mesh_scene = load(dict["mesh_scene_path"]).instantiate()
