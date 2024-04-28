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
			var spread_coordinate := coordinate + Vector2i(
				randf_range(-spread_radius/2, spread_radius/2),
				randf_range(-spread_radius/2, spread_radius/2)
			)
			spreads.emit(spread_coordinate)
			print("SPREAD!")
		
		_spread_check_timer = spread_check_rate

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["spread_check_rate"] = spread_check_rate
	dict["spread_radius"] = spread_radius
	dict["spread_chance"] = spread_chance
	dict["_spread_check_timer"] = _spread_check_timer
	dict["_active"] = _active
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	spread_check_rate = dict["spread_check_rate"] 
	spread_radius = dict["spread_radius"] 
	spread_chance = dict["spread_chance"] 
	_spread_check_timer = dict["_spread_check_timer"] 
	_active = dict["_active"] 
#endregion
