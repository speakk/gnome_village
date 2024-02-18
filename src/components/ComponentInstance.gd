class_name ComponentInstance extends Node3D

var id: Components.Id
var data: Component
var component_owner: Node3D

signal owner_set(new_owner: Node3D)

static func create_instance(component: Component, new_owner: Node3D = null) -> ComponentInstance:
	var instance: ComponentInstance
	if component.instance:
		instance = component.instance.new()
	else:
		instance = load("res://src/components/ComponentInstance.tscn").instantiate()
	instance.id = component.id
	instance.data = component
	
	if new_owner:
		instance.component_owner = new_owner
		instance.owner_set.emit(new_owner)
	
	return instance
