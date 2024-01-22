class_name HasItemRequirement extends ConditionLeaf

var item_requirement: ItemRequirement

func tick(actor: Node, blackboard: Blackboard) -> int:
	var inventory := actor.get_node("Inventory") as Inventory
	if inventory.has_item_amount(item_requirement.item_id, item_requirement.amount):
		return SUCCESS
	
	return FAILURE
