class_name InventoryComponent extends Component

@export var items_can_be_picked: bool = true
@export var pre_filled: Array[ItemRequirement]:
	set(new_value):
		pre_filled = new_value
		for item_requirement in pre_filled:
			add_item_amount(item_requirement.item, item_requirement.amount)

var item_amounts: Dictionary = {}
var reservations: Dictionary = {}

signal item_added(item: Item, amount: int)
signal item_removed(item: Item, amount: int)

func set_owner(_new_owner: Node) -> void:
	super.set_owner(_new_owner)
	for item_amount in get_items():
		item_amount.set_owner(_new_owner)

func _init() -> void:
	id = Components.Id.Inventory

func add_item_amount(item: Item, amount: int) -> void:
	if not item_amounts.has(item):
		item_amounts[item] = ItemAmountComponent.new(item)
		item_amounts[item].set_owner(component_owner)
		
	item_amounts[item].amount += amount
	item_added.emit(item, amount)

func get_item_amount(item: Items) -> ItemAmountComponent:
	return item_amounts[item]

func reserve_item_amount(item: Item, amount: int) -> void:
	if not reservations.has(item):
		reservations[item] = ItemAmountComponent.new(item)
	
	reservations[item].amount += amount

func remove_item_reservation(item: Item, amount: int) -> void:
	if not reservations.has(item):
		reservations[item] = ItemAmountComponent.new(item)
	reservations[item].amount -= amount
	if reservations[item].amount <= 0:
		reservations.erase(item)

func remove_item_amount(item: Item, amount: int) -> void:
	if not item_amounts.has(item):
		item_amounts[item] = ItemAmountComponent.new(item)
	item_amounts[item].amount -= amount
	if item_amounts[item].amount <= 0:
		item_amounts.erase(item)
	item_removed.emit(item, amount)

func has_item_amount(item: Item, amount: int) -> bool:
	if not item_amounts.has(item):
		return false
	
	var reserved: int = 0
	if reservations.has(item):
		reserved += reservations.get(item).amount
	
	return item_amounts[item].amount - reserved >= amount

func has_item_requirement(item_requirement: ItemRequirement) -> bool:
	return has_item_amount(item_requirement.item, item_requirement.amount)

func reserve_item_requirement(item_requirement: ItemRequirement) -> void:
	reserve_item_amount(item_requirement.item, item_requirement.amount)

func get_items() -> Array[ItemAmountComponent]:
	var items: Array[ItemAmountComponent]
	items.assign(item_amounts.values())
	return items
	
func save() -> Dictionary:
	return {
		item_amounts = item_amounts.values().map(func(item_amount: ItemAmountComponent) -> Dictionary: return item_amount.save())
	}

func load_save(save_dict: Dictionary) -> void:
	var item_amount_values: Array[ItemAmountComponent]
	item_amount_values.assign(save_dict["item_amounts"].map(func(save_dict: Dictionary) -> ItemAmountComponent:
		var item_amount := ItemAmountComponent.new()
		item_amount.load_save(save_dict)
		return item_amount
	))
	
	item_amounts = {}
	
	for item_amount in item_amount_values:
		item_amounts[item_amount.item] = item_amount
