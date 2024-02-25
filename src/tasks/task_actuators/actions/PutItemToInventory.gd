extends ActionLeaf

class_name PutItemToInventory

var target_inventory: InventoryComponent
var item_id: Variant
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if not target_inventory:
		print("No target inventory, FAILED")
		return FAILURE
		
	if not actor.can_reach_target(target_inventory.get_owner().global_position):
		print("Can't reach target, PUTITEM FAILED")
		return FAILURE
		
	actor.inventory.remove_item_amount(item_id, amount)
	target_inventory.add_item_amount(item_id, amount)
	return SUCCESS
