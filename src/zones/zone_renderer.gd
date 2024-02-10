extends Node3D

var ZONE_TILE_INDICATOR := preload("res://src/zones/zone_tile_indicator.tscn")

var zones: Array[Zone]:
	set(new_zones):
		zones = new_zones
		redraw()

func _ready() -> void:
	Events.zone_selected.connect(_zone_selected)
	Events.zone_updated.connect(_zone_updated)
	Events.zone_menu_hidden.connect(_zone_menu_hidden)

func _zone_selected(new_zone: Zone) -> void:
	zones = [new_zone]

func _zone_menu_hidden() -> void:
	zones = []
	
func _zone_updated(updated_zone: Zone) -> void:
	pass
	# Maybe check if updated_zone is within zones, but this is cheap so whatever
	redraw()

func redraw() -> void:
	for child in get_children():
		child.queue_free()
	
	if zones:
		for zone in zones:
			for coordinate in zone.get_coordinates():
				var tile_position := Globals.get_map().grid.map_to_local(Globals.extend_vec2i(coordinate))
				var zone_tile_indicator := ZONE_TILE_INDICATOR.instantiate()
				add_child(zone_tile_indicator)
				zone_tile_indicator.global_position = Vector3(tile_position.x, 0, tile_position.z)
