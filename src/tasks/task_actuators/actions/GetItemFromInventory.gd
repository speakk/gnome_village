extends ActionLeaf

class_name GetItemFromInventory

var target_inventory: InventoryComponent
var item: EntityDefinition
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if target_inventory == null:
		return FAILURE
	
	target_inventory.remove_item_amount(item, amount)
	actor.inventory.add_item_amount(item, amount)
	return SUCCESS
