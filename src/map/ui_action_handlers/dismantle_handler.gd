extends UiActionHandler

signal dismantle_issued(coordinate: Vector2i)

func handle_action(_ui_action: UiAction, tile_position: Vector2i, selection_draw: SelectionDraw, mouse_pressed_1: bool, mouse_pressed_2: bool) -> void:
	super.handle_action(_ui_action, tile_position, selection_draw, mouse_pressed_1, mouse_pressed_2)
	var ui_action := _ui_action as UiAction.Build

	if mouse_pressed_1:
		if Input.is_action_pressed("rectangle_select_modifier"):
			if not rect_start:
				rect_start = tile_position
			
			rect_end = tile_position
			_set_rectangle_selection(selection_draw, rect_start, rect_end)
		else:
			dismantle_issued.emit(tile_position)
			#_dismantle_in_position(tile_position)
	else:
		if rect_start and rect_end:
			for tile_coordinate in rect_tile_coords:
				dismantle_issued.emit(tile_coordinate)
				#_dismantle_in_position(tile_coordinate)
			
			clear_rectangle_selection(selection_draw)
