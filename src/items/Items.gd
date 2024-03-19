extends Node

var list: Array[Item]

func _ready() -> void:
	var paths: Array[String] = [
		"res://src/items/item_data",
		"res://src/items/item_data/plants",
		"res://src/items/item_data/food",
		"res://src/items/item_data/foliage"
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
				var item: Item
				if data is Item:
					list.append(data)
			file_name = data_dir.get_next()

func get_constructable_items() -> Array[Item]:
	var result: Array[Item]
	for item in list:
		var has_constructable := false
		for component: Component in item.components:
			if component is ConstructableComponent:
				has_constructable = true
				break
		
		if has_constructable:
			result.push_back(item)
	
	return result

func get_item_render_scene(item: Item) -> Node3D:
	var results := item.components.filter(func(component: Component) -> bool:
		return component.id == Components.Id.Scene
		)
	
	if results.size() > 0:
		var scene_component: SceneComponent = results[0]
		var scene := scene_component.scene.instantiate()
		return scene
	
	return null
