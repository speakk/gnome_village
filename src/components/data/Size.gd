class_name SizeComponent extends Component

@export var size: Vector2i = Vector2i(1, 1)

func _init() -> void:
	id = Components.Id.Size

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["x"] = size.x
	dict["y"] = size.y
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	size = Vector2i(dict["x"], dict["y"])
#endregion
