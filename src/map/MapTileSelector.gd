class_name MapTileSelector extends Node

@export var selection_draw: SelectionDraw

signal tiles_selected(coordinates: Array[Vector2i])
signal tiles_selected_secondary(coordinates: Array[Vector2i])

var selected_ui_action: UiAction

var line_start: Variant # Vector2i | null
var line_end: Variant # Vector2i | null
var line_coords: Array[Vector2i]

var rect_start: Variant # Vector2i | null
var rect_end: Variant # Vector2i | null
var rect_tile_coords: Array[Vector2i]

func _ready() -> void:
	Events.ui_action_selected.connect(func(new_ui_action: UiAction) -> void: selected_ui_action = new_ui_action)
	Events.mouse_hovered_on_map.connect(_mouse_hovered)
	

func _set_rectangle_selection(rect_start_coordinate: Vector2i, rect_end_coordinate: Vector2i, hollow: bool) -> void:
	if not rect_start_coordinate or not rect_end_coordinate:
		selection_draw.selection_rectangle = null
		return
	
	var map := Globals.get_map()
	
	# TODO: There's gotta be a better way :D
	var start_position_orig := Globals.truncate_vec3(map.grid.map_to_local(Globals.extend_vec2i(rect_start_coordinate)))
	var end_position_orig := Globals.truncate_vec3(map.grid.map_to_local(Globals.extend_vec2i(rect_end_coordinate)))
	var start_position := Vector2(min(start_position_orig.x, end_position_orig.x), min(start_position_orig.y, end_position_orig.y)) - Vector2(MainMap3D.CELL_SIZE) / 2
	var end_position := Vector2(max(start_position_orig.x, end_position_orig.x), max(start_position_orig.y, end_position_orig.y)) + Vector2(MainMap3D.CELL_SIZE) / 2
	
	var selection_rectangle := Rect2(start_position, (end_position - start_position).snapped(Vector2(MainMap3D.CELL_SIZE)))
	selection_draw.selection_rectangle = selection_rectangle
	
	var snapped_start := map.grid.local_to_map(Globals.extend_vec2(start_position + Vector2(MainMap3D.CELL_SIZE) / 2))
	var snapped_end := map.grid.local_to_map(Globals.extend_vec2(end_position - Vector2(MainMap3D.CELL_SIZE) / 2))
	
	#var snapped_start := map.grid.local_to_map(Globals.extend_vec2(start_position))
	#var snapped_end := map.grid.local_to_map(Globals.extend_vec2(end_position))
	
	var new_rect_selection_coordinates: Array[Vector2i] = []
	for y in snapped_end.z - snapped_start.z + 1:
		var real_y := y + snapped_start.z
		for x in snapped_end.x - snapped_start.x + 1:
			var real_x := x + snapped_start.x
			new_rect_selection_coordinates.append(Vector2i(real_x, real_y))
	
	var final_coordinates: Array[Vector2i]
	
	if hollow:
		var start_x := snapped_start.x
		var start_y := snapped_start.z
		var end_x := snapped_end.x
		var end_y := snapped_end.z
		final_coordinates.assign(new_rect_selection_coordinates.filter(func(coord: Vector2i) -> bool:
			return coord.x == start_x or coord.x == end_x or coord.y == start_y or coord.y == end_y)
		)	
	else:
		final_coordinates.assign(new_rect_selection_coordinates)

	rect_tile_coords = final_coordinates
	selection_draw.line_coords = final_coordinates

func clear_rectangle_selection() -> void:
	rect_start = null
	rect_end = null
	rect_tile_coords = []
	selection_draw.selection_rectangle = null

var mouse_pressed_1 := false
var mouse_pressed_2 := false

# TODO: You could use an Area2D and the input_event in that to handle this instead
func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_index == 1:
				mouse_pressed_1 = true
			else:
				mouse_pressed_2 = true
		else:
			mouse_pressed_1 = false
			mouse_pressed_2 = false
		


func _mouse_hovered(mouse_position: Vector3) -> void:
	# TODO: -0.42 is a magic number, works as (2d) "Y" offset to center the mouse
	# Probably has to do with angle of camera truncating the "y" axis by a percentage
	var tile_position:  Vector2i = Globals.truncate_vec3i(Globals.get_map().grid.local_to_map(mouse_position + Vector3(0, 0, -0.42)))
	#print("Tile position", tile_position)
	
	if not Input.is_action_pressed("line_draw_modifier"):
		line_start = null
		line_end = null
		line_coords = []
		selection_draw.line_coords = [] as Array[Vector2i]
	
	if not Input.is_action_pressed("rectangle_select_modifier"):
		clear_rectangle_selection()
	
	var check_for_solid: bool = selected_ui_action.ui_action_id == UiAction.UiActionId.Build if selected_ui_action else true
	
	#if PathFinder.is_valid_position(tile_position, check_for_solid):
	if true:
		selection_draw.line_coords = [tile_position]
		
		var signal_to_emit := tiles_selected
		
		if mouse_pressed_2:
			print("Setting secondary cause mouse2 pressed")
			signal_to_emit = tiles_selected_secondary
		
		if mouse_pressed_1 or mouse_pressed_2:
			if Input.is_action_pressed("line_draw_modifier"):
				if not line_start:
					set_line_start(tile_position)
				
				set_line_end(tile_position)
				selection_draw.line_coords = get_tile_line(line_start, line_end)
			elif not Input.is_action_pressed("rectangle_select_modifier"):
				signal_to_emit.emit([tile_position] as Array[Vector2i])
				#build_issued.emit(tile_position, item_id)
				line_start = null
				line_end = null
				
			if Input.is_action_pressed("rectangle_select_modifier"):
				if not rect_start:
					rect_start = tile_position
				
				rect_end = tile_position
				
				#var hollow := selected_ui_action.ui_action_id == UiAction.UiActionId.Build
				var hollow := selected_ui_action.ui_action_id == UiAction.UiActionId.Build if selected_ui_action else true
				
				_set_rectangle_selection(rect_start, rect_end, hollow)
			elif not Input.is_action_pressed("line_draw_modifier"):
				signal_to_emit.emit([tile_position] as Array[Vector2i])
		else:
			if rect_start and rect_end:
				signal_to_emit.emit(rect_tile_coords)
				clear_rectangle_selection()
			
			if line_start and line_end:
				var line_coords := get_tile_line(line_start, line_end)
				signal_to_emit.emit(line_coords)
			
				line_start = null
				line_end = null

#func _process(delta: float) -> void:
	#var tile_position: Vector2i = Globals.get_map().local_to_map(Globals.get_map().get_local_mouse_position())
	#print("tile pos", tile_position)

#func _process(delta: float) -> void:
	#var tile_position: Vector2i = Globals.get_map().local_to_map(Globals.get_map().get_local_mouse_position())
	##push_warning("handle_action not implemented for: %s" % _ui_action)


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
