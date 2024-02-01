extends Node

var map: MainMap

func _ready() -> void:
	Events.map_ready.connect(_map_ready)

func _map_ready(_map: MainMap) -> void:
	map = _map

func get_map() -> MainMap:
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
