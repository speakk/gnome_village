class_name WorldPositionComponent extends Component

var coordinate: Vector2i

var current_position: Vector3:
	set(new_value):
		if current_position.distance_to(new_value) > 0.01:
			Events.world_position_changed.emit(component_owner, current_position, new_value)
		current_position = new_value
		component_owner.global_position = new_value
		coordinate = Globals.get_map().global_position_to_coordinate(current_position)

static func set_world_position(node: Node3D, world_position: Vector3) -> void:
	var world_position_component: WorldPositionComponent = node.component_container.get_by_id(Components.Id.WorldPosition)
	world_position_component.current_position = world_position

func _init() -> void:
	id = Components.Id.WorldPosition
