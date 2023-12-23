extends Node

@onready var astar_grid := AStarGrid2D.new()

func _ready() -> void:
	Events.map_ready.connect(_map_ready)
	Events.solid_cell_placed.connect(_solid_cell_placed)
	Events.solid_cell_removed.connect(_solid_cell_removed)
	
func _map_ready(map: MainMap) -> void:
	astar_grid.cell_size = MainMap.CELL_SIZE
	astar_grid.region = Rect2i(0, 0, MainMap.MAP_SIZE_X, MainMap.MAP_SIZE_Y)
	astar_grid.update()

func _solid_cell_placed(coordinates: Vector2i) -> void:
	astar_grid.set_point_solid(coordinates)

func _solid_cell_removed(coordinates: Vector2i) -> void:
	astar_grid.set_point_solid(coordinates, false)

func get_point_path(from: Vector2i, to: Vector2i) -> PackedVector2Array:
	return astar_grid.get_point_path(from, to)

func get_id_path(from: Vector2i, to: Vector2i) -> PackedVector2Array:
	return astar_grid.get_id_path(from, to)

func get_point_position(id: Vector2i) -> Vector2:
	return astar_grid.get_point_position(id)
