extends PanelContainer

func _ready() -> void:
	Events.mouse_hovered_on_map.connect(_mouse_hovered)
	
func _mouse_hovered(hover_position: Vector3) -> void:
	var coord: Vector2i = Globals.get_map().global_position_to_coordinate(hover_position)
	%CoordinateLabel.text = "Coordinate: %s" % coord
