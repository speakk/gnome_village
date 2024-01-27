extends Camera2D

const CAMERA_SPEED: float = 700

func _ready() -> void:
	limit_left = 0
	limit_top = 0
	limit_right = MainMap.CELL_SIZE.x * MainMap.MAP_SIZE_X
	limit_bottom = MainMap.CELL_SIZE.y * MainMap.MAP_SIZE_Y

func _process(delta: float) -> void:
	var movement_vector: Vector2 = Vector2(0, 0)
	if Input.is_action_pressed("pan_camera_down"):
		movement_vector += Vector2(0, 1)
	if Input.is_action_pressed("pan_camera_up"):
		movement_vector += Vector2(0, -1)
	if Input.is_action_pressed("pan_camera_left"):
		movement_vector += Vector2(-1, 0)
	if Input.is_action_pressed("pan_camera_right"):
		movement_vector += Vector2(1, 0)
	
	position += movement_vector.normalized() * CAMERA_SPEED * delta
	
	var view_size := get_viewport_rect().size
	var x_offset := view_size.x / (2*zoom.x)
	var y_offset := view_size.y / (2*zoom.y)
	var x_bounds := clampf(position.x,limit_left+x_offset,limit_right-x_offset)
	var y_bounds := clampf(position.y,limit_top+y_offset,limit_bottom-y_offset)
	position = Vector2(x_bounds,y_bounds)
	
	
	#position += movement_vector.normalized() * CAMERA_SPEED * delta
	#global_position = global_position.clamp(Vector2(limit_left, limit_bottom), Vector2(limit_right, limit_top))

var max_zoom_in := 4.0
var max_zoom_out := 1.5

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				zoom = (zoom + Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				zoom = (zoom - Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
		
