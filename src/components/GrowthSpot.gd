class_name GrowthSpot extends Node3D

@onready var growth_requirement_inventory: Inventory = $Inventory

func consume_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.remove_item_amount(growth_requirement_id, amount)

func increase_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.add_item_amount(growth_requirement_id, amount)
