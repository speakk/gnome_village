extends Node

class_name Inventory

var item_amounts: Dictionary = {}

signal item_added(item_id: Variant, amount: int)
signal item_removed(item_id: Variant, amount: int)

func add_item(item: Item) -> void:
	var id: Variant = item.id
	if not item_amounts.has(id):
		item_amounts[id] = 0
	
	item_amounts[id] += item.amount
	item_added.emit(item.id, item.amount)

func remove_item(item: Item) -> void:
	item_amounts[item.id] -= item.amount
	if item_amounts[item.id] <= 0:
		item_amounts.erase(item.id)
	
	item_removed.emit(item.id, item.amount)

func add_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = 0
		
	item_amounts[item_id] += amount
	item_added.emit(item_id, amount)

func remove_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = 0
	item_amounts[item_id] -= amount
	item_removed.emit(item_id, amount)

func get_items() -> Array[Item]:
	return item_amounts.keys().map(func(key: Variant) -> Item:
		var item := Item.new()
		item.id = key
		item.amount = item_amounts[key]
		return item
	)
