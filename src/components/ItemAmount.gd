extends Node

class_name ItemAmount

signal amount_changed(new_amount: int)

var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)
		
var id: Items.Id

func _init(_amount: int = 0, _item_id: Items.Id = Items.Id.Wood) -> void:
	amount = _amount
	id = _item_id

func save() -> Dictionary:
	var save_dict := {
		"amount": amount,
		"id": id
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	amount = save_dict["amount"]
	id = save_dict["id"]