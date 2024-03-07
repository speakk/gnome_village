class_name Component extends Resource

var id: Components.Id

var component_owner: Node3D

var subscriptions: Array[Subscription]

signal removed

# Once a second
var _process_rate: float = 1.0
var _process_timer: float = _process_rate

func set_owner(_new_owner: Node3D) -> void:
	component_owner = _new_owner

func get_owner() -> Node3D:
	return component_owner

func get_container() -> ComponentContainer:
	return component_owner.component_container

func get_subscriptions() -> Array[Subscription]:
	return subscriptions

func on_exit() -> void:
	removed.emit()

func advance_process_timer(delta: float) -> bool:
	_process_timer -= delta
	if _process_timer <= 0:
		_process_timer = _process_rate
		return true
	
	return false
