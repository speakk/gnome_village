extends Node

enum Id {
	Consumable
}

var _names: Dictionary = {
	Id.Consumable: "consumable"
}

func get_group_name(id: Id) -> String:
	return _names.get(id)
