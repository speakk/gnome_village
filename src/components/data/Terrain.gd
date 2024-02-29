class_name TerrainComponent extends Component

@export var target_layer: MainMap3D.Layers = MainMap3D.Layers.Building
@export var mesh_id: MapMeshes.Id

var _blueprint_status: bool

func _init() -> void:
	id = Components.Id.Terrain
	subscriptions = [
		Subscription.new(self.id, Components.Id.WorldPosition, func (world_position: WorldPositionComponent) -> void:
			world_position.position_changed.connect(self._on_position_changed)
			)
	]

func subscribe() -> void:
	get_container().subscribe_to(Components.Id.WorldPosition, "position_changed", _on_position_changed)

func _on_position_changed(_old_position: Vector3, _global_position: Vector3, old_coordinate: Vector2i, coordinate: Vector2i) -> void:
	Events.terrain_placed.emit(coordinate, mesh_id, _blueprint_status)
	Events.terrain_cleared.emit(old_coordinate, _blueprint_status)

func set_blueprint(status: bool) -> void:
	print("Setting blueprint with new status: ", status)
	var coordinate := Globals.get_map().global_position_to_coordinate(get_owner().global_position)
	_blueprint_status = status
	Events.terrain_placed.emit(coordinate, mesh_id, status)
	Events.terrain_cleared.emit(coordinate, not status)

func on_exit() -> void:
	var coordinate := Globals.get_map().global_position_to_coordinate(get_owner().global_position)
	Events.terrain_cleared.emit(coordinate, _blueprint_status)
