extends Resource

class_name ItemRequirement

@export var item: EntityDefinition
@export var amount: int = 1

func save() -> Dictionary:
	var save_dict := {
		"item" = item,
		"amount" = amount
	}
	
	return save_dict

func _init(_item: EntityDefinition = null, _amount: int = 0) -> void:
	item = _item
	amount = _amount

func load_save(save_dict: Dictionary) -> void:
	item = save_dict["item"]
	amount = save_dict["amount"]

func get_class_name() -> String:
	return 'ItemRequirement'

func serialize() -> Dictionary:
	return {
		item = item.serialize(),
		amonut = amount
	}

func deserialize(dict: Dictionary) -> void:
	item = EntityDefinition.deserialize(dict["item"])
	amount = dict["amount"]
