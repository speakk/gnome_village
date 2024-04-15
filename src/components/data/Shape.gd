class_name ShapeComponent extends Component

@export_multiline var shape_definition: String = "1":
	set(new_value):
		shape_definition = new_value
		var rows: PackedStringArray = shape_definition.split("\n")
		width = rows[0].length()
		height = rows.size()
		size = Vector2i(width, height)
		shape = []
		for string_row in rows:
			var shape_row := ShapeRow.new()
			for character in string_row.rsplit():
				if character == "0":
					shape_row.row.append(false)
				elif character == "1":
					shape_row.row.append(true)
				else:
					push_error("Non 01 character in shape definition")
			
			shape.append(shape_row)

@export var origin: Vector2i = Vector2i(0, 0)

var width: int = 1
var height: int = 1
var size: Vector2i = Vector2i(1, 1)

var shape: Array[ShapeRow]

func get_shape() -> Array[ShapeRow]:
	return shape

func get_size() -> Vector2i:
	return size

func _init() -> void:
	id = Components.Id.Shape
	invariant = true

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["shape_definition"] = shape_definition
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	shape_definition = dict["shape_definition"]
#endregion
