class_name Subscription extends RefCounted

var subscriber_id: Components.Id
var target_id: Components.Id
var callable: Callable

func _init(_subscriber_id: Components.Id = -1, _target_id: Components.Id = -1, _callable: Callable = func() -> void: pass) -> void:
	subscriber_id = _subscriber_id
	target_id = _target_id
	callable = _callable
