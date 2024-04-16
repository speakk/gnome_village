extends Node

var list: Array[EntityDefinition]

func _ready() -> void:
	var paths: Array[String] = [
		"res://src/entities/definitions",
		"res://src/entities/definitions/plants",
		"res://src/entities/definitions/food",
		"res://src/entities/definitions/foliage",
		"res://src/entities/definitions/stations"
		]
	for path in paths:
		var data_dir := DirAccess.open(path)
		data_dir.list_dir_begin()
		var file_name := data_dir.get_next()
		print("File name: ", file_name)
		while file_name != "":
			if not data_dir.current_is_dir():
				var data := load("%s/%s" % [path, file_name])
				print("Loaded: %s in %s" % [file_name, path])
				var item: EntityDefinition
				if data is EntityDefinition:
					list.append(data)
			file_name = data_dir.get_next()

func get_constructable_items() -> Array[EntityDefinition]:
	var result: Array[EntityDefinition]
	for item in list:
		var has_constructable := false
		for component: Component in item.components:
			if component is ConstructableComponent:
				has_constructable = true
				break
		
		if has_constructable:
			result.push_back(item)
	
	return result

func get_item_render_scene(item: EntityDefinition) -> Node3D:
	var scene_component: SceneComponent = item.get_component_by_id(Components.Id.Scene)
	if scene_component:
		var scene := scene_component.scene.instantiate()
		return scene
	
	return null
