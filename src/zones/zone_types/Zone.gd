class_name Zone extends Node2D

var zone_name: String
var zone_type: ZoneManager.ZoneType

func _ready() -> void:
	# Ensure zone is in zone group because group inheritance doesn't work
	add_to_group("zone")

func set_zone_name(_zone_name: String) -> void:
	zone_name = _zone_name

func clean_up() -> void:
	push_warning("Zone clean up not implemented for a zone")
