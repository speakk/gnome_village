extends Node

class_name ItemAmount

signal amount_changed(new_amount: int)

var amount: int = 1:
	set(new_amount):
		amount = new_amount
		amount_changed.emit(new_amount)
		
var id: Items.Id
