extends Camera3D

const CAMERA_SPEED: float = 15
const MAX_SPEED: float = 2

var position_target: Vector3
var velocity: Vector3

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

	velocity += Vector3(movement_vector.x, 0, movement_vector.y).normalized() * CAMERA_SPEED * delta / Engine.time_scale
	velocity = velocity.limit_length(MAX_SPEED)
	
	var size_speed_multiplier: float = size / max_size
	position += velocity * size_speed_multiplier
	
	velocity = velocity.move_toward(Vector3(), delta * 10)
	
	
	var ray_result: Variant = _get_ray_result(get_viewport().get_mouse_position())
	if ray_result is Vector3:
		Events.mouse_hovered_on_map.emit(ray_result)
	
	var rectpos: Vector2 = get_viewport().get_visible_rect().size / 2
	var ray_result2: Variant = _get_ray_result(rectpos)
	if ray_result2 is Vector3:
		#$MeshInstance3D.global_position = Vector3(ray_result2.x, 0.5, ray_result2.z)
		$AudioListener3D.global_position = Vector3(ray_result2.x, $AudioListener3D.global_position.y, ray_result2.z)
		
var zoom_step := 2.5
var min_size := 7.0
var max_size := 77.0
var default_size := 37.0

var reverb_multiplier_max:float = 0.7

func _ready() -> void:
	_set_size(default_size)
	position_target = global_position

func _set_size(new_size: float) -> void:
	size = new_size
	$AudioListener3D.position.y = new_size * 1.2 - min_size
	AudioServer.get_bus_effect(1, 0).wet = size / max_size * 0.2

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			_select_target(event.position)
	
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				_set_size(size - zoom_step)
				#zoom = (zoom + Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				_set_size(size + zoom_step)
				#zoom = (zoom - Vector2(0.5, 0.5)).clamp(Vector2(max_zoom_out, max_zoom_out), Vector2(max_zoom_in, max_zoom_in))
		


@warning_ignore("untyped_declaration")
func _get_ray_result(hover_position: Vector2):
	var from := project_ray_origin(hover_position)
	var to := from + project_ray_normal(hover_position)*6000
	var space_state := get_world_3d().direct_space_state
	var ray_query := PhysicsRayQueryParameters3D.create(from, to)
	ray_query.collide_with_areas = true
	ray_query.collide_with_bodies = false
	#ray_query.collision_mask = 0b00000000_00000000_00000000_00000010
	var raycast_result := space_state.intersect_ray(ray_query)
	if raycast_result.has("position"):
		return raycast_result.get("position") as Vector3
	
		#print("HOVER POS")
		#Events.mouse_hovered_on_map.emit(raycast_result.get("position") as Vector3)
	

func _select_target(click_position: Vector2) -> void:
	var from := project_ray_origin(click_position)
	var to := from + project_ray_normal(click_position)*6000
	var space_state := get_world_3d().direct_space_state
	var ray_query := PhysicsRayQueryParameters3D.create(from, to)
	ray_query.collide_with_areas = true
	ray_query.collide_with_bodies = false
	#ray_query.collision_mask = 0b00000000_00000000_00000000_00000010
	var raycast_result := space_state.intersect_ray(ray_query)
	#print("Result", raycast_result)
	if raycast_result.has("position"):
		Events.mouse_clicked_on_map.emit(raycast_result.get("position") as Vector3)
