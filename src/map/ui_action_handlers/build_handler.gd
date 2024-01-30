extends UiActionHandler

signal build_issued(coordinate: Vector2i, item_id: Items.Id)

func handle_action(_ui_action: UiAction, tile_position: Vector2i, selection_draw: SelectionDraw, mouse_pressed_1: bool, mouse_pressed_2: bool) -> void:
	super.handle_action(_ui_action, tile_position, selection_draw, mouse_pressed_1, mouse_pressed_2)
	var ui_action := _ui_action as UiAction.Build
	var item_id := ui_action.item_id
	if item_id:
		if PathFinder.is_valid_position(tile_position):
			selection_draw.line_coords = [tile_position]
			
			if mouse_pressed_1:
				if Input.is_action_pressed("line_draw_modifier"):
					if not line_start:
						set_line_start(tile_position)
					
					set_line_end(tile_position)
					selection_draw.line_coords = get_tile_line(line_start, line_end)
				else:
					build_issued.emit(tile_position, item_id)
					#_place_blueprint(tile_position, item_id)
					line_start = null
					line_end = null
			else:
				if line_start and line_end:
					var line_coords := get_tile_line(line_start, line_end)
					for line_coord in line_coords:
						build_issued.emit(line_coord, item_id)
						#_place_blueprint(line_coord, item_id)
				
					line_start = null
					line_end = null
