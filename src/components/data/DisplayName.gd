class_name DisplayNameComponent extends Component

@export var display_name: String

func _init(new_display_name: String = "Bob") -> void:
	id = Components.Id.DisplayName
	display_name = new_display_name
	
