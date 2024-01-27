extends Node2D

var line_coords: Array[Vector2i]

func set_line_coords(_line_coords: Array[Vector2i]) -> void:
	line_coords = _line_coords
	queue_redraw()

func _draw() -> void:
	if line_coords and line_coords.size() > 0:
		for line_coord in line_coords:
			var tile_position := Globals.get_map().map_to_local(line_coord)
			var rect := Rect2(tile_position - Vector2(MainMap.CELL_SIZE) / 2, MainMap.CELL_SIZE)
			draw_rect(rect, Color.BISQUE, false, 1)
