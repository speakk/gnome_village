extends Node2D

var is_locked := false
var open_amount := 0.0

func open_by_amount(amount: float) -> void:
	open_amount += amount
	if open_amount >= 1:
		open_amount = 1

func is_fully_open() -> bool:
	return open_amount >= 1.0
