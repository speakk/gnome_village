extends Camera2D

const CAMERA_SPEED: float = 700

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
