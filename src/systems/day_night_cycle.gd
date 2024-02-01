extends Node2D

const DAY_SPEED: float = 0.01

# Current time between 0 and 1. 0.5 is the middle of the day
var current_time: float = 0.3:
	set(new_value):
		Events.current_time_changed.emit(new_value)
		current_time = new_value

func _process(delta: float) -> void:
	current_time += delta * DAY_SPEED
	#current_time = wrapf(current_time + current_time * delta * DAY_SPEED, 0.0, 1.0)
	current_time = wrapf(current_time, 0.0, 1.0)
