class_name ComponentInstance extends Node3D

var id: Components.Id
var data: Component
var component_owner: Node

static func create_instance_by_id(component_id: Components.Id, new_owner: Node = null) -> ComponentInstance:
	var component := Components.create_component_by_id(component_id)
	var instance: ComponentInstance = load("res://src/components/ComponentInstance.tscn").instantiate()
	instance.id = component.id
	instance.data = component
	
	if new_owner:
		instance.component_owner = new_owner
	
	return instance

static func create_instance(component: Component, new_owner: Node = null) -> ComponentInstance:
	var instance: ComponentInstance = load("res://src/components/ComponentInstance.tscn").instantiate()
	instance.id = component.id
	instance.data = component
	
	if new_owner:
		instance.component_owner = new_owner
	
	return instance
