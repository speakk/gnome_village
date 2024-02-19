extends MarginContainer

var component_ui_priority_list: Array[Components.Id] = [Components.Id.DisplayName, Components.Id.CharacterStats]

func set_entity(entity: Node3D) -> void:
	var component_container: ComponentContainer = entity.component_container
	var all_component_instances := component_container.get_all()
	all_component_instances.sort_custom(func(a: ComponentInstance, b: ComponentInstance) -> bool:
		var index1 := component_ui_priority_list.find(a.id)
		var index2 := component_ui_priority_list.find(b.id)
		index1 = index1 if index1 >= 0 else 99999
		index2 = index2 if index2 >= 0 else 99999
		return index1 < index2
		)
	
	for child in %ComponentInfos.get_children():
		child.queue_free()
	
	for component_instance in all_component_instances:
		var component_display := ComponentDisplays.get_component_display(component_instance.id)
		if component_display:
			component_display.set_component(component_instance.data)
			%ComponentInfos.add_child(component_display)
