class_name ComponentContainer extends Node3D

@onready var component_owner := get_parent()
@export var default_components: Array[Component]

var _components: Array[Component]

func _ready() -> void:
	for component in default_components:
		add_component(component)

func has_component(component_id: Components.Id) -> bool:
	return _components.filter(func(component: Component) -> bool:
		return component.id == component_id
	).size() > 0
		
func get_by_id(component_id: Components.Id) -> Component:
	return _components.filter(func(component: Component) -> bool:
		return component.id == component_id
	)[0]

func get_all() -> Array[Component]:
	var all: Array[Component]
	all.assign(_components)
	return all

func add_component(component: Component) -> void:
	var duplicated := component.duplicate()
	duplicated.set_owner(component_owner)
	_components.append(duplicated)

func clear() -> void:
	_components.clear()

func _process(delta: float) -> void:
	for component in get_all():
		if component.has_method("process_component"):
			component.process_component(delta)
