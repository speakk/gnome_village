class_name DoorComponent extends Component

const SELF_CLOSE_DELAY := 2.0
const SELF_CLOSE_SPEED := 1.2
var self_close_timer := 0.0

signal open_amount_changed(new_amount: float)

func _init() -> void:
	id = Components.Id.Door
	_process_rate = 0.1

var is_locked := false
var open_amount := 0.0:
	set(new_amount):
		open_amount = new_amount
		open_amount_changed.emit(open_amount)

func process_component(delta: float) -> void:
	if open_amount > 0:
		if self_close_timer <= 0:
			open_amount -= SELF_CLOSE_SPEED * delta

	self_close_timer -= delta


func open_by_amount(amount: float) -> void:
	self_close_timer = SELF_CLOSE_DELAY
	open_amount += amount
	if open_amount >= 1:
		open_amount = 1

func is_open() -> bool:
	return open_amount >= 1.0
