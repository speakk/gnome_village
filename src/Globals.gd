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
