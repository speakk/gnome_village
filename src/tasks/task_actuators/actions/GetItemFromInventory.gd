extends ActionLeaf

class_name GetItemFromInventory

var target_inventory: Inventory
var item_id: Variant
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if target_inventory == null:
		return FAILURE
	
	target_inventory.remove_item_amount(item_id, amount)
	actor.inventory.add_item_amount(item_id, amount)
	return SUCCESS
