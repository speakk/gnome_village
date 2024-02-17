extends Node

class_name Inventory

var item_amounts: Dictionary = {}

var reservations: Dictionary = {}

signal item_added(item_id: Variant, amount: int)
signal item_removed(item_id: Variant, amount: int)

func add_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = ItemAmount.new(0, item_id)
		
	item_amounts[item_id].amount += amount
	item_added.emit(item_id, amount)

func reserve_item_amount(item_id: Variant, amount: int) -> void:
	if not reservations.has(item_id):
		reservations[item_id] = ItemAmount.new(0, item_id)
	
	reservations[item_id].amount += amount

func remove_item_reservation(item_id: Variant, amount: int) -> void:
	if not reservations.has(item_id):
		reservations[item_id] = ItemAmount.new(0, item_id)
	reservations[item_id].amount -= amount
	if reservations[item_id].amount <= 0:
		reservations.erase(item_id)

func remove_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = ItemAmount.new(0, item_id)
	item_amounts[item_id].amount -= amount
	if item_amounts[item_id].amount <= 0:
		item_amounts.erase(item_id)
	item_removed.emit(item_id, amount)

func has_item_amount(item_id: Variant, amount: int) -> bool:
	if not item_amounts.has(item_id):
		return false
	
	var reserved: int = 0
	if reservations.has(item_id):
		reserved += reservations.get(item_id).amount
	
	return item_amounts[item_id].amount - reserved >= amount

func has_item_requirement(item_requirement: ItemRequirement) -> bool:
	return has_item_amount(item_requirement.item_id, item_requirement.amount)

func reserve_item_requirement(item_requirement: ItemRequirement) -> void:
	reserve_item_amount(item_requirement.item_id, item_requirement.amount)

func get_items() -> Array[ItemAmount]:
	var items: Array[ItemAmount]
	items.assign(item_amounts.values())
	return items
	
func save() -> Dictionary:
	return {
		item_amounts = item_amounts.values().map(func(item_amount: ItemAmount) -> Dictionary: return item_amount.save())
	}

func load_save(save_dict: Dictionary) -> void:
	var item_amount_values: Array[ItemAmount]
	item_amount_values.assign(save_dict["item_amounts"].map(func(save_dict: Dictionary) -> ItemAmount:
		var item_amount := ItemAmount.new()
		item_amount.load_save(save_dict)
		return item_amount
	))
	
	item_amounts = {}
	
	for item_amount in item_amount_values:
		item_amounts[item_amount.id] = item_amount
