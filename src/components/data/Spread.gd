class_name SpreadComponent extends Component

@export var spread_check_rate: float = 5.0
@export var spread_radius: int = 10.0
@export_range(0.0, 1.0, 0.01) var spread_chance: float = 0.1

signal spreads(coordinate: Vector2i)

var _spread_check_timer := spread_check_rate
var _active := true

func _init() -> void:
	id = Components.Id.Spread

func set_active(new_value: bool) -> void:
	_active = new_value

func process_component(delta: float) -> void:
	if not _active:
		return
		
	_spread_check_timer -= delta
	if _spread_check_timer <= 0:
		if randf() < spread_chance:
			var owner_coordinate: Vector2i = get_container().get_by_id(Components.Id.WorldPosition).coordinate
			var spread_coordinate := owner_coordinate + Vector2i(
				randf_range(-spread_radius/2, spread_radius/2),
				randf_range(-spread_radius/2, spread_radius/2)
			)
			spreads.emit(spread_coordinate)
			print("SPREAD!")
		
		_spread_check_timer = spread_check_rate
