class_name Component extends Resource

var id: Components.Id

var component_owner: Object

var subscriptions: Array[Subscription]

#var groups: Array[Groups.Id]:
	#set(new_value):
		#groups = new_value
		#if component_owner:
			#for group in groups:
				#component_owner.add_to_group(Groups.get_group_name(group))

signal removed
var _being_deleted: bool = false

# Once a second
var _process_rate: float = 1.0
var _process_timer: float = _process_rate

func set_owner(_new_owner: Object) -> void:
	component_owner = _new_owner
	#for group in groups:
		#component_owner.add_to_group(Groups.get_group_name(group))

func get_owner() -> Node3D:
	return component_owner

func get_container() -> ComponentContainer:
	return component_owner.component_container

func get_subscriptions() -> Array[Subscription]:
	return subscriptions

func on_exit() -> void:
	_being_deleted = true
	removed.emit()

var _full_delta: float = 0

func advance_process_timer(delta: float) -> void:
	_process_timer -= delta
	_full_delta += delta
	if _process_timer <= 0:
		_process_timer = _process_rate
		if has_method("process_component"):
			call("process_component", _full_delta)
		_full_delta = 0

func serialize() -> Dictionary:
	var save_dict: Dictionary
	save_dict["resource_path"] = get_script().get_path()
		
	return {
		resource_path = get_script().get_path()
	}

static func static_deserialize(dict: Dictionary) -> Component:
	var component: Component = load(dict["resource_path"]).new()
	return component

func deserialize(dict: Dictionary) -> void:
	pass
