extends ActionLeaf

class_name GetItemAmount

var target_item: ItemOnGround
var requirement_item_id: Items.Id
var amount: int

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	# amount * 2 because at this point we assume the existing reservation in the
	# inventory is because we reserved it
	# Fixes not being able to get last amount from inventory
	# TODO: Make reservations objects and add "reserved_by" to it along with amount
	if not target_item or \
	(target_item.item_amount.amount < amount) \
	or (not target_item and not target_item.inventory.has_item_amount(requirement_item_id, amount * 2)):
		print("FAILED at GetItemAmount", target_item, name)
		return FAILURE
	
	if not actor.can_reach_target(target_item.global_position):
		print("Failed at GetItemAmount")
		return FAILURE
	
	if target_item.item_id == requirement_item_id:
		target_item.item_amount.amount -= amount
		target_item.reserved_for_picking = false
	else:
		target_item.inventory.remove_item_amount(requirement_item_id, amount)
		target_item.inventory.remove_item_reservation(requirement_item_id, amount)
	
	print("Got amount, remaining amount:", amount, target_item.item_amount.amount)
	actor.inventory.add_item_amount(requirement_item_id, amount)
	return SUCCESS
