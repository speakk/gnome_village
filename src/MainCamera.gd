extends Camera3D

const CAMERA_SPEED: float = 15
const MAX_SPEED: float = 2

var position_target: Vector3
var velocity: Vector3

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
	
	_position_audio_listener()
	_clamp_camera_to_world_bounds()
		
var zoom_step := 2.5
var min_size := 7.0
var max_size := 60.0
var default_size := 37.0

var reverb_multiplier_max:float = 0.7

var last_valid_x: float = 0
var last_valid_y: float = 0
var last_valid_ray_right: Variant
var last_valid_ray_center: Variant

const ground_plane := Plane.PLANE_XZ

func _clamp_camera_to_world_bounds() -> void:
	var plane_intersection: Rect2 = get_frustum_plane_intersection(ground_plane, self)
	if plane_intersection.position.y < -MainMap3D.MAP_SIZE_Y/2:
		var diff := - plane_intersection.position.y - MainMap3D.MAP_SIZE_Y/2
		global_position.z += diff
		
	if plane_intersection.position.y + plane_intersection.size.y > MainMap3D.MAP_SIZE_Y/2:
		var diff := plane_intersection.position.y + plane_intersection.size.y - MainMap3D.MAP_SIZE_Y/2
		global_position.z -= diff
	
	if plane_intersection.position.x < -MainMap3D.MAP_SIZE_X/2:
		var diff := - plane_intersection.position.x - MainMap3D.MAP_SIZE_X/2
		global_position.x += diff
		
	if plane_intersection.position.x + plane_intersection.size.x > MainMap3D.MAP_SIZE_X/2:
		var diff := plane_intersection.position.x + plane_intersection.size.x - MainMap3D.MAP_SIZE_X/2
		global_position.x -= diff
		

func _position_audio_listener() -> void:
	var rectpos: Vector2 = get_viewport().get_visible_rect().size / 2
	var ray_result2: Variant = _get_ray_result(rectpos)
	if ray_result2 is Vector3:
		$AudioListener3D.global_position = Vector3(ray_result2.x, $AudioListener3D.global_position.y, ray_result2.z)

func _ready() -> void:
	_set_size(default_size)
	position_target = global_position

func _set_size(new_size: float) -> void:
	size = clampf(new_size, min_size, max_size)
	$AudioListener3D.position.y = new_size * 1.2 - min_size
	AudioServer.get_bus_effect(1, 0).wet = size / max_size * 0.2

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			var ray_result: Variant = _get_ray_result(event.position)
			if ray_result:
				Events.mouse_clicked_on_map.emit(ray_result)
			
	
	if event is InputEventMouseButton:
		if event.pressed:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				_set_size(size - zoom_step)
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				_set_size(size + zoom_step)
		


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

static func get_frustum_plane_intersection(plane: Plane, camera: Camera3D) -> Variant:
	var side_planes : Array[Plane] = camera.get_frustum()
	var near_plane : Plane = side_planes.pop_front()
	var far_plane : Plane = side_planes.pop_front()
	var result : Array[Vector3] = []
	for i in side_planes.size():
		var near_point : Vector3 = side_planes[i-1].intersect_3(side_planes[i], near_plane)
		var far_point : Vector3 = side_planes[i-1].intersect_3(side_planes[i], far_plane)
		var intersection : Vector3 = plane.intersects_segment(near_point, far_point)
		# Is null if plane is either behind near_point or further than far_point
		if intersection == null:
			return null
		result.append(intersection)
	
	var min_x: float = [result[0].x, result[1].x, result[2].x, result[3].x].min()
	var max_x: float = [result[0].x, result[1].x, result[2].x, result[3].x].max()
	var min_z: float = [result[0].z, result[1].z, result[2].z, result[3].z].min()
	var max_z: float = [result[0].z, result[1].z, result[2].z, result[3].z].max()
	
	return Rect2(Vector2(min_x, min_z), Vector2(max_x - min_x, max_z - min_z))
	
	#return 
	#return result
