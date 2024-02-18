extends Node

enum Id {
	Selectable, Door
}

var component_by_id: Dictionary

func _ready() -> void:
	var data_dir := DirAccess.open("res://src/components/data")
	data_dir.list_dir_begin()
	var file_name := data_dir.get_next()
	while file_name != "":
		if not data_dir.current_is_dir():
			var component_data: Component = load("res://src/components/data/%s" % file_name)
			if component_by_id.has(component_data.id):
				push_error("Component Id duplicate found: ", component_data.id, file_name)
			component_by_id[component_data.id] = component_data
		file_name = data_dir.get_next()

func create_component_by_id(id: Id) -> Component:
	return component_by_id[id].duplicate()
