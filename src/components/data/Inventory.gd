class_name InventoryComponent extends Component

@export var items_can_be_picked: bool = false
@export var pre_filled: Array[ItemRequirement]:
	set(new_value):
		pre_filled = new_value
		for item_requirement in pre_filled:
			add_item_amount(item_requirement.item, item_requirement.amount)

var item_amounts: Dictionary = {}
var reservations: Dictionary = {}

signal item_added(item: EntityDefinition, amount: int)
signal item_removed(item: EntityDefinition, amount: int)

func set_owner(_new_owner: Object) -> void:
	super.set_owner(_new_owner)
	for item_amount in get_items():
		item_amount.set_owner(_new_owner)

func _init() -> void:
	id = Components.Id.Inventory

func add_item_amount(item: EntityDefinition, amount: int) -> void:
	assert(item)
	if not item_amounts.has(item):
		item_amounts[item] = ItemAmountComponent.new(item)
		item_amounts[item].set_owner(component_owner)
		
	item_amounts[item].amount += amount
	item_added.emit(item, amount)

func get_item_amount(item: EntityDefinition) -> ItemAmountComponent:
	return item_amounts[item]

func reserve_item_amount(item: EntityDefinition, amount: int) -> void:
	if not reservations.has(item):
		reservations[item] = ItemAmountComponent.new(item)
	
	reservations[item].amount += amount

func remove_item_reservation(item: EntityDefinition, amount: int) -> void:
	if not reservations.has(item):
		reservations[item] = ItemAmountComponent.new(item)
	reservations[item].amount -= amount
	if reservations[item].amount <= 0:
		reservations.erase(item)

func remove_item_amount(item: EntityDefinition, amount: int) -> void:
	if not item_amounts.has(item):
		item_amounts[item] = ItemAmountComponent.new(item)
	item_amounts[item].amount -= amount
	if item_amounts[item].amount <= 0:
		item_amounts.erase(item)
	item_removed.emit(item, amount)

func has_item_amount(item: EntityDefinition, amount: int) -> bool:
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
	
#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["item_amounts"] = item_amounts.values().map(
		func(item_amount: ItemAmountComponent) -> Dictionary:
			return item_amount.serialize()
			)
	dict["items_can_be_picked"] = items_can_be_picked
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	if dict.has("items_can_be_picked"):
		items_can_be_picked = dict["items_can_be_picked"]
	else:
		items_can_be_picked = false
	
	var item_amount_values: Array[ItemAmountComponent]
	item_amount_values.assign(dict["item_amounts"].map(func(item_amount_dict: Dictionary) -> ItemAmountComponent:
		var item_amount := ItemAmountComponent.new()
		item_amount.deserialize(item_amount_dict)
		item_amount.set_owner(component_owner)
		return item_amount
	))
	
	for item_amount in get_items():
		remove_item_amount(item_amount.item, item_amount.amount)
	
	for item_amount in item_amount_values:
		add_item_amount(item_amount.item, item_amount.amount)
		
#endregion
