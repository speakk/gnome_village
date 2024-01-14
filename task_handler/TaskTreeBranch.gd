extends Node

class_name TaskTreeBranch

enum OrderType {
	Sequence, Parallel
}

var order_type: OrderType = OrderType.Sequence
var root: bool = false

func clean_up() -> void:
	for child in get_children():
		if child.has_method("clean_up"):
			print("Calling clean up on: ", child)
			child.clean_up()
		
		child.queue_free()
	
	queue_free()
