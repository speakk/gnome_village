extends Node

class_name Inventory

class InventoryItemAmount:
	var amount: int
	var item_id: Items.Id
	
	func _init(_amount: int = 0, _item_id: Items.Id = Items.Id.Wood) -> void:
		amount = _amount
		item_id = item_id

var item_amounts: Dictionary = {}

signal item_added(item_id: Variant, amount: int)
signal item_removed(item_id: Variant, amount: int)

func add_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = InventoryItemAmount.new(0, item_id)
		
	item_amounts[item_id].amount += amount
	item_added.emit(item_id, amount)

func remove_item_amount(item_id: Variant, amount: int) -> void:
	if not item_amounts.has(item_id):
		item_amounts[item_id] = InventoryItemAmount.new(0, item_id)
	item_amounts[item_id].amount -= amount
	item_removed.emit(item_id, amount)

func get_items() -> Array[Item]:
	return item_amounts.values()
