class_name TerrainComponent extends Component

@export var target_layer: MainMap.Layers = MainMap.Layers.Building
@export var mesh_id: MainMap.AboveGroundCells

var _cached_coordinate: Vector2i

var _blueprint_status: bool:
	set(new_status):
		if not _being_deleted:
			_blueprint_status = new_status
			Events.terrain_placed.emit(_cached_coordinate, mesh_id, _blueprint_status)
			Events.terrain_cleared.emit(_cached_coordinate, not _blueprint_status)

func _init() -> void:
	id = Components.Id.Terrain
	subscriptions = [
		Subscription.new(self.id, Components.Id.WorldPosition, func (world_position: WorldPositionComponent) -> void:
			_cached_coordinate = world_position.coordinate
			if not world_position.position_changed.is_connected(self._on_position_changed):
				world_position.position_changed.connect(self._on_position_changed)
			),
		Subscription.new(self.id, Components.Id.Blueprint, func (blueprint: BlueprintComponent) -> void:
			_blueprint_status = true
			blueprint.removed.connect(func() -> void:
				_blueprint_status = false
				)
			)
	]

func _on_position_changed(_old_position: Vector3, _global_position: Vector3, old_coordinate: Vector2i, coordinate: Vector2i) -> void:
	Events.terrain_placed.emit(coordinate, mesh_id, _blueprint_status)
	Events.terrain_cleared.emit(old_coordinate, _blueprint_status)
	
	_cached_coordinate = coordinate

func set_blueprint(status: bool) -> void:
	_blueprint_status = status

func on_exit() -> void:
	super.on_exit()
	Events.terrain_cleared.emit(_cached_coordinate, _blueprint_status)
	Events.terrain_cleared.emit(_cached_coordinate, !_blueprint_status)

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["target_layer"] = target_layer
	dict["mesh_id"] = mesh_id
	dict["_blueprint_status"] = _blueprint_status
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	target_layer = dict["target_layer"]
	mesh_id = dict["mesh_id"]
	_blueprint_status = dict["_blueprint_status"]
	
#endregion
