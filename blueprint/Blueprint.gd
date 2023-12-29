extends Node2D

class_name Blueprint

@onready var ITEM_ON_GROUND := preload("res://items/ItemOnGround.tscn")

var build_progress := 0.0
var item_id: Items.Id

var finish_emitted := false

func initialize(_item_id: Items.Id) -> Blueprint:
	item_id = _item_id
	$Sprite2D.modulate = Color(0.2, 0.2, 1.0, 0.2)
	$ProgressBar.value = build_progress
	return self

func finish_construction() -> void:
	if not finish_emitted:
		#Events.solid_cell_placed.emit(Globals.get_map().global_position_to_coordinate(global_position))
		$Sprite2D.modulate = Color.WHITE
		$ProgressBar.hide()
		
		await get_tree().process_frame
		
		var item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_id, 1)
		item_on_ground.global_position = global_position
		get_parent().add_child(item_on_ground)
		
		print("Finish construction")
		finish_emitted = true
		Events.blueprint_finished.emit(self)

func increase_build_progress(amount: float) -> void:
	build_progress += amount
	$ProgressBar.value = build_progress
	if is_finished():
		finish_construction()

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
