extends Node2D

class_name Blueprint

var build_progress := 0.0
var item_id: Items.Id

func initialize(_item_id: Items.Id) -> Blueprint:
	item_id = _item_id
	$Sprite2D.modulate = Color(0.2, 0.2, 1.0, 0.2)
	$ProgressBar.value = build_progress
	return self

func increase_build_progress(amount: float) -> void:
	build_progress += amount
	$ProgressBar.value = build_progress
	if is_finished():
		Events.blueprint_finished.emit(self)
		Events.solid_cell_placed.emit(Globals.get_map().local_to_map(global_position))
		$Sprite2D.modulate = Color.WHITE
		$ProgressBar.hide()

func is_finished() -> bool:
	return build_progress >= 1.0

func has_materials() -> bool:
	var material_requirements := Items.get_crafting_requirements(item_id)
	
	for requirement in material_requirements:
		var deposited := $Inventory.get_items().find(func(depo: Inventory.InventoryItemAmount) -> bool: return depo.id == requirement.item_id) as Inventory.InventoryItemAmount
		if deposited.amount < requirement.amount:
			return false
	#for deposited_item in $Inventory.get_items() as Array[Item]:
		
	
	return true
