class_name Item extends Resource

@export var display_name: String = ""
@export var components: Array[Component]

func has_component(component_id: Components.Id) -> bool:
	return components.filter(func(component: Component) -> bool:
		return component.id == component_id
	).size() > 0

func get_component_by_id(component_id: Components.Id) -> Component:
	return components.filter(func(component: Component) -> bool:
		return component.id == component_id
	)[0]
