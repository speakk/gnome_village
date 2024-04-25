class_name ItemAmountComponent extends Component

signal amount_changed(new_amount: int)

var item: EntityDefinition

var _reservations: Array[ItemAmountReservation]

# TODO: Make some variant for this without "item"
# Just a kind of AmountComponent or something

@export var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)
		if Events:
			Events.component.item_amount_changed.emit(self)

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

func has_item_amount(_item: EntityDefinition, _amount: int) -> bool:
	if item != _item:
		return false
	
	var reserved: int = 0
	for reservation in _reservations:
		reserved += reservation.amount
	
	return amount - reserved >= _amount

func has_item_requirement(item_requirement: ItemRequirement) -> bool:
	return has_item_amount(item_requirement.item, item_requirement.amount)

func _init(_item: EntityDefinition = null, _amount: int = 0) -> void:
	amount = _amount
	item = _item
	id = Components.Id.ItemAmount

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["amount"] = amount
	dict["item"] = item.serialize()
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	amount = dict["amount"]
	item = EntityDefinition.deserialize(dict["item"])
#endregion
