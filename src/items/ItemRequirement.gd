extends Resource

class_name ItemRequirement

@export var item: Item
@export var amount: int = 1

func save() -> Dictionary:
	var save_dict := {
		"item" = item,
		"amount" = amount
	}
	
	return save_dict

func _init(_item: Item = null, _amount: int = 0) -> void:
	item = _item
	amount = _amount

func load_save(save_dict: Dictionary) -> void:
	item = save_dict["item"]
	amount = save_dict["amount"]

func get_class_name() -> String:
	return 'ItemRequirement'
