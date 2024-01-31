extends Node2D

var zones: Array[Zone]:
	set(new_zones):
		zones = new_zones
		queue_redraw()

func _ready() -> void:
	Events.zone_selected.connect(_zone_selected)
	Events.zone_updated.connect(_zone_updated)
	Events.zone_menu_hidden.connect(_zone_menu_hidden)

func _zone_selected(new_zone: Zone) -> void:
	zones = [new_zone]

func _zone_menu_hidden() -> void:
	zones = []
	
func _zone_updated(updated_zone: Zone) -> void:
	# Maybe check if updated_zone is within zones, but this is cheap so whatever
	queue_redraw()

func _draw() -> void:
	if zones:
		for zone in zones:
			for coordinate in zone.get_coordinates():
				var tile_position := Globals.get_map().map_to_local(coordinate)
				var rect := Rect2(tile_position - Vector2(MainMap.CELL_SIZE) / 2, MainMap.CELL_SIZE)
				draw_rect(rect, Color(0.5, 1.0, 0.1, 0.2), true, 1)
	
