extends Camera3D

const CAMERA_SPEED: float = 70

@export var target: Node3D

@onready var ray := $RayCast3D

func _process(delta: float) -> void:
	var movement_vector: Vector2 = Vector2(0, 0)
	if not Globals.is_focus_in_control():
		if Input.is_action_pressed("pan_camera_down"):
			movement_vector += Vector2(0, 1)
		if Input.is_action_pressed("pan_camera_up"):
			movement_vector += Vector2(0, -1)
		if Input.is_action_pressed("pan_camera_left"):
			movement_vector += Vector2(-1, 0)
		if Input.is_action_pressed("pan_camera_right"):
			movement_vector += Vector2(1, 0)

	position += Vector3(movement_vector.x, 0, movement_vector.y).normalized() * CAMERA_SPEED * delta / Engine.time_scale

	_hover_target(get_viewport().get_mouse_position())

var max_zoom_in := 4.0
var max_zoom_out := 1.5
var zoom_step := 2.5

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			_select_target(event.position)
	
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				size -= zoom_step
				#zoom = (zoom + Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				size += zoom_step
				#zoom = (zoom - Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
		


func _hover_target(hover_position: Vector2) -> void:
	var from := project_ray_origin(hover_position)
	var to := from + project_ray_normal(hover_position)*6000
	var space_state := get_world_3d().direct_space_state
	var ray_query := PhysicsRayQueryParameters3D.create(from, to)
	ray_query.collide_with_areas = true
	ray_query.collide_with_bodies = false
	#ray_query.collision_mask = 0b00000000_00000000_00000000_00000010
	var raycast_result := space_state.intersect_ray(ray_query)
	if raycast_result.has("position"):
		Events.mouse_hovered_on_map.emit(raycast_result.get("position") as Vector3)

func _select_target(click_position: Vector2) -> void:
	var from := project_ray_origin(click_position)
	var to := from + project_ray_normal(click_position)*6000
	var space_state := get_world_3d().direct_space_state
	var ray_query := PhysicsRayQueryParameters3D.create(from, to)
	ray_query.collide_with_areas = true
	ray_query.collide_with_bodies = false
	#ray_query.collision_mask = 0b00000000_00000000_00000000_00000010
	var raycast_result := space_state.intersect_ray(ray_query)
	print("Result", raycast_result)
	if raycast_result.has("position"):
		Events.mouse_clicked_on_map.emit(raycast_result.get("position") as Vector3)
