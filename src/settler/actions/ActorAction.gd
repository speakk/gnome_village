class_name ActorAction extends Resource

var _start_processed: bool

var _started: bool

var actor: Settler

signal finished()
signal validation_failed()

func process_action(delta: float) -> void:
	push_warning("ActorAction process_action called, did you forget to implement?")

func _init(actor: Settler, params: Dictionary) -> void:
	_started = true
	validate(actor, params)

func is_started() -> bool:
	return _started

func validate(actor: Settler, params: Dictionary) -> void:
	pass

func set_settler(settler: Settler) -> void:
	actor = settler
