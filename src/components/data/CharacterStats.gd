@tool
class_name CharacterStatsComponent extends Component

enum Id {
	Happiness, Melancholy, Hunger, Thirst, Tiredness
}

class CharacterStat:
	var stat_id: Id
	var display_name: String
	var value: float
	
	func _init(_stat_id: Id, _display_name: String, _value: float = 0) -> void:
		stat_id = _stat_id
		display_name = _display_name
		value = _value

var stats: Array[CharacterStat] = [
	CharacterStat.new(Id.Happiness, "Happiness"),
	CharacterStat.new(Id.Melancholy, "Melancholy"),
	CharacterStat.new(Id.Hunger, "Hunger"),
	CharacterStat.new(Id.Thirst, "Thirst"),
	CharacterStat.new(Id.Tiredness, "Tiredness"),
]

func _init() -> void:
	id = Components.Id.CharacterStats
	
func get_stats() -> Array[CharacterStat]:
	return stats
