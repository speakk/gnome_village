extends MarginContainer

var component_ui_priority_list: Array[Components.Id] = [Components.Id.DisplayName, Components.Id.CharacterStats]

func set_entity(entity: Entity) -> void:
	var component_container: ComponentContainer = entity.component_container
	var all_components := component_container.get_all()
	all_components.sort_custom(func(a: Component, b: Component) -> bool:
		var index1 := component_ui_priority_list.find(a.id)
		var index2 := component_ui_priority_list.find(b.id)
		index1 = index1 if index1 >= 0 else 99999
		index2 = index2 if index2 >= 0 else 99999
		return index1 < index2
		)
	
	for child in %ComponentInfos.get_children():
		child.queue_free()
	
	for component in all_components:
		var component_display := ComponentDisplays.get_component_display(component.id)
		if component_display:
			component_display.set_component(component)
			%ComponentInfos.add_child(component_display)
