class_name ActorAction extends Node

var _start_processed: bool

signal finished(actor_action: ActorAction)

func process_action(actor: Settler, delta: float) -> void:
	push_warning("ActorAction process_action called, did you forget to implement?")
