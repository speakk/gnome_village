class_name ShapeComponent extends Component

@export_multiline var shape_definition: String:
	set(new_value):
		shape_definition = new_value
		var rows: PackedStringArray = shape_definition.split("\n")
		for string_row in rows:
			var shape_row := ShapeRow.new()
			for character in string_row.rsplit():
				if character == "0":
					shape_row.row.append(false)
				elif character == "1":
					shape_row.row.append(true)
				else:
					push_error("Non 01 character in shape definition")

@export var origin: Vector2i = Vector2i(0, 0)

var shape: Array[ShapeRow]

func get_shape() -> Array[ShapeRow]:
	return shape

func _init() -> void:
	id = Components.Id.Shape

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["shape_definition"] = shape_definition
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	shape_definition = dict["shape_definition"]
#endregion
