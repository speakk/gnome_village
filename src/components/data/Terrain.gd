class_name TerrainComponent extends Component

@export var target_layer: MainMap3D.Layers = MainMap3D.Layers.Building
@export var mesh_id: MapMeshes.Id

func _init() -> void:
	id = Components.Id.Terrain

func set_blueprint(status: bool) -> void:
	print("Setting blueprint with new status: ", status)
	var coordinate := Globals.get_map().global_position_to_coordinate(get_owner().global_position)
	Events.terrain_placed.emit(coordinate, mesh_id, status)
	Events.terrain_cleared.emit(coordinate, not status)

func on_exit() -> void:
	var coordinate := Globals.get_map().global_position_to_coordinate(get_owner().global_position)
	Events.terrain_cleared.emit(coordinate, true)
	Events.terrain_cleared.emit(coordinate, false)
