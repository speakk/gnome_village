extends ActionLeaf

class_name GetItemAmount

var item_amount_component: ItemAmountComponent
var item_requirement: ItemRequirement

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	# amount * 2 because at this point we assume the existing reservation in the
	# inventory is because we reserved it
	# Fixes not being able to get last amount from inventory
	# TODO: Make reservations objects and add "reserved_by" to it along with amount
	if not item_amount_component or item_amount_component.amount * 2 < item_requirement.amount:
		print("FAILED at GetItemAmount", item_amount_component, name)
		return FAILURE
	
	item_amount_component.amount -= item_requirement.amount
	
	var inventory: InventoryComponent = actor.component_container.get_by_id(Components.Id.Inventory)
	inventory.add_item_amount(item_requirement.item, item_requirement.amount)
	return SUCCESS
