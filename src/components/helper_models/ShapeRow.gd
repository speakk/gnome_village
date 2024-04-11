class_name ShapeRow extends Resource

@export var row: Array[bool]

#region Serialization
func serialize() -> Dictionary:
	var dict := {}
	dict["row"] = row
	return dict

func deserialize(dict: Dictionary) -> void:
	row = dict["row"]
#endregion
