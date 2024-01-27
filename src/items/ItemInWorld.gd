extends Node

class_name ItemInWorld

var item_type: Item

signal amount_changed(new_amount: int)

var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)
		
#var id: Variant
