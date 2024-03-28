extends Resource

class_name ItemDrop

@export var item: EntityDefinition
@export var amount_min: int = 1
@export var amount_max: int = 1
@export var probability: float = 1.0

#region Serialization
func serialize() -> Dictionary:
	return {
		item = item.serialize(),
		amount_min = amount_min,
		amount_max = amount_max,
		probability = probability,
	}

func deserialize(dict: Dictionary) -> void:
	item = EntityDefinition.deserialize(dict["item"])
	amount_min = dict["amount_min"]
	amount_max = dict["amount_max"]
	probability = dict["probability"]
#endregion
