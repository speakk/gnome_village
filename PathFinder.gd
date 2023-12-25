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

func is_position_solid(coordinates: Vector2i) -> bool:
	return astar_grid.is_point_solid(coordinates)

var all_directions: Array[Vector2i] = [
	Vector2i(-1, -1),
	Vector2i(0, -1),
	Vector2i(1, -1),
	Vector2i(1, 0),
	Vector2i(1, 1),
	Vector2i(0, 1),
	Vector2i(-1, 1),
	Vector2i(-1, 0),
]

func get_surrounding_coordinates(center_coordinate: Vector2i) -> Array[Vector2i]:
	var surrounding: Array[Vector2i] = []
	for direction in all_directions:
		var coordinate := center_coordinate + direction
		surrounding.push_back(coordinate)
	
	return surrounding

# TODO: This only checks immediate surrounding tiles, no further
func get_closest_free_point(coordinate: Vector2i) -> Variant:
	var surrounding_coordinates := get_surrounding_coordinates(coordinate)
	surrounding_coordinates.shuffle()
	for surrounding_coordinate in surrounding_coordinates:
		if not is_position_solid(surrounding_coordinate):
			return surrounding_coordinate
	
	return null

func get_id_path(from: Vector2i, to: Vector2i) -> PackedVector2Array:
	#print("Getting id path: ", from, to)
	var found_path := astar_grid.get_id_path(from, to)
	#if found_path:
	#	found_path.pop_back()
	
	return found_path
	#return astar_grid.get_id_path(from, to)

func get_point_position(id: Vector2i) -> Vector2:
	return astar_grid.get_point_position(id)
