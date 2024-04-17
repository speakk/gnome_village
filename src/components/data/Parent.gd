class_name ParentComponent extends Component

var inherit_transform: bool
var delete_on_parent_delete: bool

var parent: Entity:
	set(new_value):
		parent = new_value
		if delete_on_parent_delete:
			parent.delete_called.connect(func() -> void:
				get_owner().delete()
				)


func _init() -> void:
	id = Components.Id.Parent

func on_enter() -> void:
	var parent_position_component: WorldPositionComponent = parent.component_container.get_by_id(Components.Id.WorldPosition)
	var position_component: WorldPositionComponent = get_container().get_by_id(Components.Id.WorldPosition)
	position_component.current_position = parent_position_component.current_position
	
	if inherit_transform:
		parent_position_component.position_changed.connect(func(_old_position: Vector3, global_position: Vector3, _old_coordinate: Vector2i, _coordinate: Vector2i) -> void:
			position_component.current_position = global_position
			)
		
