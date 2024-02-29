class_name SolidComponent extends Component

func _init() -> void:
	id = Components.Id.Solid

	subscriptions = [
		Subscription.new(self.id, Components.Id.WorldPosition, func (world_position: WorldPositionComponent) -> void:
			world_position.position_changed.connect(self._on_position_changed)
			Events.solid_cell_placed.emit(world_position.coordinate)
			)
	]

func _on_position_changed(_old_position: Vector3, _global_position: Vector3, old_coordinate: Vector2i, coordinate: Vector2i) -> void:
	Events.solid_cell_placed.emit(coordinate)
	Events.solid_cell_removed.emit(old_coordinate)

func on_enter() -> void:
	Events.solid_cell_placed.emit(get_container().get_by_id(Components.Id.WorldPosition).coordinate)

func on_exit() -> void:
	Events.solid_cell_removed.emit(get_container().get_by_id(Components.Id.WorldPosition).coordinate)
