extends Node

var map: MainMap3D

func _ready() -> void:
	Events.map_ready.connect(_map_ready)

func _map_ready(_map: MainMap3D) -> void:
	map = _map

func get_map() -> MainMap3D:
	return map

var control_has_focus: bool = false

func register_focus_input(input: Control) -> void:
	input.focus_entered.connect(func() -> void: control_has_focus = true)
	input.focus_exited.connect(func() -> void: control_has_focus = false)
	
func is_focus_in_control() -> bool:
	return control_has_focus


static func weighted_random(weights: Array[float]) -> int:
	var weights_sum := 0.0
	for weight in weights:
		weights_sum += weight
	
	var remaining_distance := randf() * weights_sum
	for i in weights.size():
		remaining_distance -= weights[i]
		if remaining_distance < 0:
			return i
	
	return 0

func truncate_vec3(vector: Vector3) -> Vector2:
	return Vector2(vector.x, vector.z)

func extend_vec2(vector: Vector2) -> Vector3:
	return Vector3(vector.x, 0, vector.y)

func truncate_vec3i(vector: Vector3i) -> Vector2i:
	return Vector2i(vector.x, vector.z)

func extend_vec2i(vector: Vector2i) -> Vector3i:
	return Vector3i(vector.x, 0, vector.y)
