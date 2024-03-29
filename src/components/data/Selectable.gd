class_name SelectableComponent extends Component

var selected: bool:
	set(new_value):
		selected = new_value
		if new_value:
			Events.entity_selected.emit(component_owner)
		else:
			Events.entity_deselected.emit(component_owner)
			
func _init() -> void:
	id = Components.Id.Selectable

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["selected"] = selected
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	selected = dict["selected"]
#endregion
