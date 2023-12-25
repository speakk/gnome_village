extends ActionLeaf

class_name GetItemAmount

var target_item: Item
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if target_item == null or target_item.amount < amount:
		print("FAILED at GetItemAmount", target_item)
		return FAILURE
	
	if not actor.is_next_to_target(target_item.get_parent().global_position):
		return FAILURE
	
	target_item.amount -= amount
	actor.get_node("Inventory").add_item_amount(target_item.id, amount)
	return SUCCESS
