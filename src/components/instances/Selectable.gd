extends ComponentInstance

var selected: bool:
	set(new_value):
		selected = new_value
		if new_value:
			Events.entity_selected.emit(component_owner)
		else:
			Events.entity_deselected.emit(component_owner)
			
