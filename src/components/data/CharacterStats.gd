@tool
class_name CharacterStatsComponent extends Component

enum Id {
	Happiness, Melancholy, Hunger, Thirst, Tiredness
}

# How fast stats move, here to make the value_deltas a bit more palatable
const DELTA_MULTIPLIER: float = 0.05

class CharacterStat:
	var stat_id: Id
	var display_name: String
	var value: float
	var value_delta: float
	
	func _init(_stat_id: Id = -1, _display_name: String = "", _value: float = 0, _value_delta:float = 0.01) -> void:
		stat_id = _stat_id
		display_name = _display_name
		value = _value
		value_delta = _value_delta

var stats: Dictionary = {
	Id.Happiness: CharacterStat.new(Id.Happiness, "Happiness", 1.0, -0.01),
	Id.Melancholy: CharacterStat.new(Id.Melancholy, "Melancholy", 0.0, randf_range(0, 0.03)),
	Id.Hunger: CharacterStat.new(Id.Hunger, "Hunger", randf_range(0.01, 0.03)),
	Id.Thirst: CharacterStat.new(Id.Thirst, "Thirst", randf_range(0, 0.03)),
	Id.Tiredness: CharacterStat.new(Id.Tiredness, "Tiredness", randf_range(0, 0.03)),
}

func _randomize_deltas() -> void:
	get_stat(Id.Happiness).value_delta = -0.01
	get_stat(Id.Melancholy).value_delta = randf_range(0.01, 0.03)
	get_stat(Id.Hunger).value_delta = randf_range(0.01, 0.03)
	get_stat(Id.Thirst).value_delta = randf_range(0.01, 0.03)
	get_stat(Id.Tiredness).value_delta = randf_range(0.1, 0.2)

func _init() -> void:
	id = Components.Id.CharacterStats
	
func on_enter() -> void:
	_randomize_deltas()

func get_stats() -> Array[CharacterStat]:
	var stats_array: Array[CharacterStat]
	stats_array.assign(stats.values())
	return stats_array

func get_stat(id: Id) -> CharacterStat:
	if stats.has(id):
		return stats[id]
	
	return null

func apply_satisfactions(satisfactions: Array[Satisfaction]) -> void:
	for satisfaction in satisfactions:
		apply_stat_amount(satisfaction.character_stat, satisfaction.amount)

func apply_stat_amount(stat_id: Id, amount: float) -> void:
	var stat := get_stat(stat_id)
	stat.value += amount

func process_component(delta: float) -> void:
	for stat: CharacterStat in stats.values():
		stat.value += stat.value_delta * delta * DELTA_MULTIPLIER
		stat.value = clampf(stat.value, 0.0, 1.0)
		

func serialize() -> Dictionary:
	var dict := super.serialize()
	return dict
