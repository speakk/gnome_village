class_name ItemAmountComponent extends Component

signal amount_changed(new_amount: int)

var item_id: Items.Id

var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)
		

func _init(_amount: int = 0, _item_id: Items.Id = Items.Id.Wood) -> void:
	amount = _amount
	item_id = _item_id
	id = Components.Id.ItemAmount

func save() -> Dictionary:
	var save_dict := {
		"amount": amount,
		"item_id": item_id
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	amount = save_dict["amount"]
	item_id = save_dict["item_id"]
