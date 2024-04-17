class_name ActorAction extends Resource

var _start_processed: bool

var _started: bool

var actor: Settler

signal finished()
signal cancelled
signal validation_failed()

func process_action(delta: float) -> void:
	push_warning("ActorAction process_action called, did you forget to implement?")

func _init(_actor: Settler) -> void:
	_started = true
	actor = _actor

func is_started() -> bool:
	return _started

func validate(actor: Settler, task: Task) -> void:
	pass
