class_name HasItemRequirement extends ConditionLeaf

var item_requirement: ItemRequirement

func tick(actor: Node, blackboard: Blackboard) -> int:
	var inventory: InventoryComponent = actor.component_container.get_by_id(Components.Id.Inventory)
	if inventory.has_item_amount(item_requirement.item, item_requirement.amount):
		return SUCCESS
	
	return FAILURE
