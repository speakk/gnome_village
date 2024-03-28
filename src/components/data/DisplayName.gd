class_name DisplayNameComponent extends Component

@export var display_name: String

func _init(new_display_name: String = "Bob") -> void:
	id = Components.Id.DisplayName
	display_name = new_display_name


#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["display_name"] = display_name
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	display_name = dict["display_name"]
#endregion
