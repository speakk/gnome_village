extends Sprite2D

@onready var original_rotation := rotation_degrees

const SELF_CLOSE_TIME := 2.0
const SELF_CLOSE_SPEED := 0.5
var self_close_timer := 0.0

var is_locked := false
var open_amount := 0.0:
	set(new_amount):
		rotation_degrees = original_rotation + new_amount * 90
		#$LightOccluder2D.ro
		open_amount = new_amount

func open_by_amount(amount: float) -> void:
	self_close_timer = SELF_CLOSE_TIME
	open_amount += amount
	if open_amount >= 1:
		open_amount = 1

func is_open() -> bool:
	return open_amount >= 1.0

func _physics_process(delta: float) -> void:
	if open_amount > 0:
		if self_close_timer <= 0:
			open_amount -= SELF_CLOSE_SPEED * delta

	self_close_timer -= delta
