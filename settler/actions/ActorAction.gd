class_name ActorAction extends Node

signal finished(actor_action: ActorAction)

func process_action(actor: Settler, delta: float) -> void:
	push_error("Action process function is abstract - Forgot to implement it for an action?")
