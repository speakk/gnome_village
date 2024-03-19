class_name SelectableComponent extends Component

var selected: bool:
	set(new_value):
		selected = new_value
		if new_value:
			Events.entity_selected.emit(component_owner)
		else:
			Events.entity_deselected.emit(component_owner)
			
func _init() -> void:
	push_warning("init selectable")
	id = Components.Id.Selectable
