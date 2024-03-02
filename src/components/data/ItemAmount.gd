class_name ItemAmountComponent extends Component

signal amount_changed(new_amount: int)

var item_id: Items.Id

var _reservations: Array[ItemAmountReservation]

var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)

func add_reservation(item_amount_reservation: ItemAmountReservation) -> void:
	_reservations.append(item_amount_reservation)

func remove_reservation(item_amount_reservation: ItemAmountReservation) -> void:
	if _reservations.has(item_amount_reservation):
		_reservations.erase(item_amount_reservation)
	else:
		for reservation in _reservations:
			if reservation.reserved_by == item_amount_reservation.reserved_by:
				_reservations.erase(reservation)
				break

func has_item_amount(_item_id: Variant, _amount: int) -> bool:
	if item_id != _item_id:
		return false
	
	return amount >= _amount

func has_item_requirement(item_requirement: ItemRequirement) -> bool:
	return has_item_amount(item_requirement.item_id, item_requirement.amount)

func _init(_item_id: Items.Id = Items.Id.Wood, _amount: int = 0) -> void:
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
