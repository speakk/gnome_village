extends Node2D

@onready var original_rotation := rotation_degrees

var is_locked := false
var open_amount := 0.0:
	set(new_amount):
		rotation_degrees = original_rotation + new_amount * 90
		open_amount = new_amount

func open_by_amount(amount: float) -> void:
	open_amount += amount
	if open_amount >= 1:
		open_amount = 1

func is_fully_open() -> bool:
	return open_amount >= 1.0
