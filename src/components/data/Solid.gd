class_name SolidComponent extends Component

var shape_component: ShapeComponent

func _init() -> void:
	id = Components.Id.Solid
	invariant = true

	subscriptions = [
		Subscription.new(self.id, Components.Id.WorldPosition, func (world_position: WorldPositionComponent) -> void:
			world_position.position_changed.connect(self._on_position_changed)
			_change_cell_state(world_position.coordinate, true)
			),
		Subscription.new(self.id, Components.Id.Shape, func (_shape_component: ShapeComponent) -> void:
			shape_component = _shape_component
			)
	]

func _change_cell_state(coordinate: Vector2i, placed: bool) -> void:
	var _default_shape_row := ShapeRow.new()
	_default_shape_row.row = [true]
	var shape: Array[ShapeRow] = [_default_shape_row]
	var origin: Vector2i
	
	if shape_component:
		shape = shape_component.get_shape()
		origin = shape_component.origin
	
	for y in shape.size():
		var shape_row: ShapeRow = shape[y]
		for x in shape_row.row.size():
			var cell: bool = shape_row.row[x]
			if cell:
				if placed:
					Events.solid_cell_placed.emit(coordinate - origin + Vector2i(x, y))
				else:
					Events.solid_cell_removed.emit(coordinate - origin + Vector2i(x, y))

func _on_position_changed(_old_position: Vector3, _global_position: Vector3, old_coordinate: Vector2i, coordinate: Vector2i) -> void:
	_change_cell_state(coordinate, true)
	_change_cell_state(old_coordinate, false)

func on_enter() -> void:
	_change_cell_state(get_container().get_by_id(Components.Id.WorldPosition).coordinate, true)
	

func on_exit() -> void:
	super.on_exit()
	_change_cell_state(get_container().get_by_id(Components.Id.WorldPosition).coordinate, false)
	
