class_name ItemAmountReservation extends RefCounted

var reserved_by: Node
var amount: int

func _init(_reserved_by: Node, _amount: int) -> void:
	reserved_by = _reserved_by
	amount = _amount
