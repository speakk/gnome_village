class_name UiActionHandler extends RefCounted

var line_start: Variant # Vector2i | null
var line_end: Variant # Vector2i | null
var line_coords: Array[Vector2i]

var rect_start: Variant # Vector2i | null
var rect_end: Variant # Vector2i | null
var rect_tile_coords: Array[Vector2i]

func _set_rectangle_selection(selection_draw: SelectionDraw, rect_start_coordinate: Vector2i, rect_end_coordinate: Vector2i) -> void:
	if not rect_start_coordinate or not rect_end_coordinate:
		selection_draw.selection_rectangle = null
		return
	
	var map := Globals.get_map()
	
	# TODO: There's gotta be a better way :D
	var start_position_orig := Vector2(map.map_to_local(rect_start_coordinate))
	var end_position_orig := Vector2(map.map_to_local(rect_end_coordinate))
	var start_position := Vector2(min(start_position_orig.x, end_position_orig.x), min(start_position_orig.y, end_position_orig.y)) - Vector2(MainMap.CELL_SIZE) / 2
	var end_position := Vector2(max(start_position_orig.x, end_position_orig.x), max(start_position_orig.y, end_position_orig.y)) + Vector2(MainMap.CELL_SIZE) / 2
	var selection_rectangle := Rect2(start_position, (end_position - start_position).snapped(Vector2(MainMap.CELL_SIZE)))
	selection_draw.selection_rectangle = selection_rectangle
	
	var snapped_start := map.local_to_map(start_position + Vector2(MainMap.CELL_SIZE) / 2)
	var snapped_end := map.local_to_map(end_position - Vector2(MainMap.CELL_SIZE) / 2)
	
	var new_rect_selection_coordinates: Array[Vector2i] = []
	for y in snapped_end.y - snapped_start.y + 1:
		var real_y := y + snapped_start.y
		for x in snapped_end.x - snapped_start.x + 1:
			var real_x := x + snapped_start.x
			new_rect_selection_coordinates.append(Vector2i(real_x, real_y))
	
	rect_tile_coords = new_rect_selection_coordinates
	print("Rect tile coords set", rect_tile_coords)

func clear_rectangle_selection(selection_draw: SelectionDraw) -> void:
	rect_start = null
	rect_end = null
	rect_tile_coords = []
	selection_draw.selection_rectangle = null

func handle_action(_ui_action: UiAction, tile_position: Vector2i, selection_draw: SelectionDraw, mouse_pressed_1: bool, mouse_pressed_2: bool) -> void:
	#push_warning("handle_action not implemented for: %s" % _ui_action)
	if not Input.is_action_pressed("line_draw_modifier"):
		line_start = null
		line_end = null
		line_coords = []
		selection_draw.line_coords = [] as Array[Vector2i]
	
	if not Input.is_action_pressed("rectangle_select_modifier"):
		clear_rectangle_selection(selection_draw)

func set_line_start(coordinate: Vector2i) -> void:
	line_start = coordinate
	
func set_line_end(coordinate: Vector2i) -> void:
	line_end = coordinate

#Bresenham's line algorithm
func get_tile_line(from: Vector2i, to: Vector2i) -> Array[Vector2i]:
	var points: Array[Vector2i] = []
	var dx := absi(to.x - from.x)
	var dy := -absi(to.y - from.y)
	var err := dx + dy
	var e2 := 2 * err
	var sx := 1 if from.x < to.x else -1
	var sy := 1 if from.y < to.y else -1
	while true:
		points.append(Vector2i(from.x, from.y))
		if from.x == to.x and from.y == to.y:
			break
		e2 = 2 * err
		if e2 >= dy:
			err += dy
			from.x += sx
		if e2 <= dx:
			err += dx
			from.y += sy
	return points
