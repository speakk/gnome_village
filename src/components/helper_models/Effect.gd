class_name Effect extends Resource

@export var effect_scene: PackedScene

#region Serialization
func serialize() -> Dictionary:
	var dict := {}
	dict["effect_scene_path"] = effect_scene.resource_path
		
	return dict

func deserialize(dict: Dictionary) -> void:
	effect_scene = load(dict["effect_scene_path"])

#endregion
