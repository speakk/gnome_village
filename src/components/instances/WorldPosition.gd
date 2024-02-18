class_name WorldPosition extends ComponentInstance

var current_position: Vector3:
	set(new_value):
		if current_position.distance_to(new_value) > 0.01:
			Events.world_position_changed.emit(component_owner, current_position, new_value)
		current_position = new_value
		component_owner.global_position = new_value

static func set_world_position(node: Node3D, world_position: Vector3) -> void:
	var world_position_component: WorldPosition = node.component_container.get_component_instance(Components.Id.WorldPosition) as WorldPosition
	world_position_component.current_position = world_position
