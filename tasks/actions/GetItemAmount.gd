extends ActionLeaf

class_name GetItemAmount

var target_item: ItemOnGround
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if target_item == null or target_item.itemAmount.amount < amount:
		print("FAILED at GetItemAmount", target_item)
		return FAILURE
	
	if not actor.can_reach_target(target_item.global_position):
		print("Failed at GetItemAmount")
		return FAILURE
	
	target_item.itemAmount.amount -= amount
	actor.get_node("Inventory").add_item_amount(target_item.item_id, amount)
	return SUCCESS
