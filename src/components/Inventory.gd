extends Node

class_name Inventory

var item_amounts: Dictionary = {}

signal item_added(item_id: Variant, amount: int)
signal item_removed(item_id: Variant, amount: int)

func add_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = ItemAmount.new(0, item_id)
		
	item_amounts[item_id].amount += amount
	item_added.emit(item_id, amount)

func remove_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = ItemAmount.new(0, item_id)
	item_amounts[item_id].amount -= amount
	item_removed.emit(item_id, amount)

func has_item_amount(item_id: Variant, amount: int) -> bool:
	if not item_amounts.has(item_id):
		return false
	
	return item_amounts[item_id].amount >= amount

func get_items() -> Array[Item]:
	return item_amounts.values()
	
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
