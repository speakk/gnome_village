class_name ComponentContainer extends Node3D

@onready var component_owner := get_parent()

func has_component(component_id: Components.Id) -> bool:
	return get_children().filter(func(child: Node) -> bool:
		if child is ComponentInstance:
			return child.id == component_id
		return false
		).size() > 0
		

func get_component_instance(component_id: Components.Id) -> ComponentInstance:
	return get_children().filter(func(child: Node) -> bool:
		if child is ComponentInstance:
			return child.id == component_id
		return false
		)[0]

func get_all() -> Array[ComponentInstance]:
	var all: Array[ComponentInstance]
	all.assign(get_children())
	return all

func add_component(component: Component) -> void:
	var component_instance := ComponentInstance.create_instance(component, component_owner)
	add_child(component_instance)

func clear() -> void:
	for child in get_children():
		child.queue_free()

func _process(delta: float) -> void:
	for component_instance in get_all():
		if component_instance.data.has_method("process_component"):
			component_instance.data.process_component(delta)
