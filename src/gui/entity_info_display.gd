extends PanelContainer

func set_entity(entity: Node3D) -> void:
	var component_container: ComponentContainer = entity.component_container
	if component_container.has_component(Components.Id.DisplayName):
		var display_name: String = component_container.get_component_instance(Components.Id.DisplayName).data.display_name
		%EntityName.text = display_name
