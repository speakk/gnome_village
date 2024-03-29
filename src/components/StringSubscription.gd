class_name StringSubscription extends Resource

@export var target_id: Components.Id
@export var method_name: String

func serialize() -> Dictionary:
	return {
		target_id = target_id,
		method_name = method_name
	}

func deserialize(dict: Dictionary) -> void:
	target_id = dict["target_id"]
	method_name = dict["method_name"]
