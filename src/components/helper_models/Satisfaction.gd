class_name Satisfaction extends Resource

@export var character_stat: CharacterStatsComponent.Id
@export var amount: float

func serialize() -> Dictionary:
	return {
		character_stat = character_stat,
		amount = amount
	}

func deserialize(dict: Dictionary) -> void:
	character_stat = dict["character_stat"]
	amount = dict["amount"]
