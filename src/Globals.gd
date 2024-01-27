extends Node

var map: MainMap

func _ready() -> void:
	Events.map_ready.connect(_map_ready)

func _map_ready(_map: MainMap) -> void:
	map = _map

func get_map() -> MainMap:
	return map
