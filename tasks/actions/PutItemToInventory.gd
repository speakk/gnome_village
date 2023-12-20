extends ActionLeaf

class_name PutItemToInventory

var target_inventory: Inventory
var item_id: Variant
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	actor.get_node("Inventory").remove_item_amount(item_id, amount)
	target_inventory.add_item_amount(item_id, amount)
	return SUCCESS
