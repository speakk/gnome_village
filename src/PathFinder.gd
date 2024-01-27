extends Node

@onready var astar_grid := AStarGrid2D.new()

func _ready() -> void:
	Events.map_ready.connect(_map_ready)
	Events.solid_cell_placed.connect(_solid_cell_placed)
	Events.solid_cell_removed.connect(_solid_cell_removed)

func prepare_for_load() -> void:
	astar_grid = AStarGrid2D.new()
	_map_ready(null)

func _map_ready(_map: MainMap) -> void:
	astar_grid.cell_size = MainMap.CELL_SIZE
	astar_grid.region = Rect2i(0, 0, MainMap.MAP_SIZE_X, MainMap.MAP_SIZE_Y)
	astar_grid.diagonal_mode = AStarGrid2D.DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES
	astar_grid.update()

func _solid_cell_placed(coordinates: Vector2i) -> void:
	astar_grid.set_point_solid(coordinates)

func _solid_cell_removed(coordinates: Vector2i) -> void:
	astar_grid.set_point_solid(coordinates, false)

func get_point_path(from: Vector2i, to: Vector2i) -> PackedVector2Array:
	return astar_grid.get_point_path(from, to)

func is_position_solid(coordinates: Vector2i) -> bool:
	return astar_grid.is_point_solid(coordinates)

func is_valid_position(coordinate: Vector2i) -> bool:
	return astar_grid.is_in_bounds(coordinate.x, coordinate.y) and not is_position_solid(coordinate)

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
	var found_path := astar_grid.get_id_path(from, to)
	return found_path

func get_id_path_to_closest_point(from: Vector2i, to: Vector2i) -> PackedVector2Array:
	var found_path := astar_grid.get_id_path(from, to)
	if not found_path:
		var direction := Vector2(from).direction_to(Vector2(to))
		var all_directions_clone := all_directions.duplicate()
		all_directions_clone.sort_custom(func(a: Vector2i, b: Vector2i) -> bool:
				return Vector2(a).angle_to(direction) < Vector2(b).angle_to(direction)
		)
		for new_direction in all_directions_clone as Array[Vector2i]:
			var new_path := astar_grid.get_id_path(from, to + new_direction)
			if new_path:
				return new_path
		
	return found_path

func get_point_position(id: Vector2i) -> Vector2:
	return astar_grid.get_point_position(id)

func set_coordinate_invalid(coordinate: Vector2i) -> void:
	astar_grid.set_point_solid(coordinate)
