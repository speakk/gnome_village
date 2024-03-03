extends Resource

class_name ItemRequirement

@export var item_id: Items.Id = Items.Id.Wood
@export var amount: int = 1

func save() -> Dictionary:
	var save_dict := {
		"item_id" = item_id,
		"amount" = amount
	}
	
	return save_dict

func _init(_item_id: Items.Id = -1, _amount: int = 0) -> void:
	item_id = _item_id
	amount = _amount

func load_save(save_dict: Dictionary) -> void:
	item_id = save_dict["item_id"]
	amount = save_dict["amount"]

func get_class_name() -> String:
	return 'ItemRequirement'
