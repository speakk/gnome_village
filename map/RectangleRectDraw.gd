extends Node2D

var selection_rectangle: Variant:
	set(new_value):
		selection_rectangle = new_value
		queue_redraw()

func _draw() -> void:
	if selection_rectangle:
		draw_rect(selection_rectangle, Color.BISQUE, false, 1)
