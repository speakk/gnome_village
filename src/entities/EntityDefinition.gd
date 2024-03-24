class_name EntityDefinition extends Resource

@export var display_name: String = ""
@export var components: Array[Component]

func has_component(component_id: Components.Id) -> bool:
	return components.filter(func(component: Component) -> bool:
		return component.id == component_id
	).size() > 0

func get_component_by_id(component_id: Components.Id) -> Component:
	var matching_components: Array[Component] = components.filter(func(component: Component) -> bool:
		return component.id == component_id
	)
	
	if matching_components.size() > 0:
		return matching_components[0]
	
	return null
