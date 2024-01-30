class_name SelectionDraw extends Node2D

var line_coords: Array[Vector2i]:
	set(new_value):
		line_coords = new_value
		queue_redraw()

var selection_rectangle: Variant:
	set(new_value):
		selection_rectangle = new_value
		queue_redraw()

func _draw() -> void:
	if line_coords and line_coords.size() > 0:
		for line_coord in line_coords:
			var tile_position := Globals.get_map().map_to_local(line_coord)
			var rect := Rect2(tile_position - Vector2(MainMap.CELL_SIZE) / 2, MainMap.CELL_SIZE)
			draw_rect(rect, Color.BISQUE, false, 1)
	
	if selection_rectangle:
		draw_rect(selection_rectangle, Color.BISQUE, false, 1)
