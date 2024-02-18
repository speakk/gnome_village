@tool
class_name CharacterStatsComponent extends Component

enum Id {
	Happiness, Melancholy, Hunger, Thirst, Tiredness
}

class CharacterStat:
	var stat_id: Id
	var display_name: String
	var value: float
	var value_delta: float
	
	func _init(_stat_id: Id, _display_name: String, _value: float = 0, _value_delta:float = 0.01) -> void:
		stat_id = _stat_id
		display_name = _display_name
		value = _value
		value_delta = _value_delta

var stats: Array[CharacterStat] = [
	CharacterStat.new(Id.Happiness, "Happiness", 1.0, -0.01),
	CharacterStat.new(Id.Melancholy, "Melancholy", 0.0, randf_range(0, 0.03)),
	CharacterStat.new(Id.Hunger, "Hunger", randf_range(0, 0.03)),
	CharacterStat.new(Id.Thirst, "Thirst", randf_range(0, 0.03)),
	CharacterStat.new(Id.Tiredness, "Tiredness", randf_range(0, 0.03)),
]

func _init() -> void:
	id = Components.Id.CharacterStats
	
func get_stats() -> Array[CharacterStat]:
	return stats

func process_component(delta: float) -> void:
	for stat in stats:
		stat.value += stat.value_delta * delta
		stat.value = clampf(stat.value, 0.0, 1.0)
		
	
