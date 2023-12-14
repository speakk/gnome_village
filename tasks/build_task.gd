extends Task

class_name BuildTask

var target_tile: Vector2i

func initialize(_target_tile: Vector2i) -> BuildTask:
	target_tile = _target_tile
	return self
