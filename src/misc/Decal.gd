extends Sprite2D

func _ready() -> void:
	frame = [1,2,3][Globals.weighted_random([0.02, 0.6, 0.6])]
