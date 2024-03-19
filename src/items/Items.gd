extends Node

enum Id {
	Stone, Wood, WoodenWall, Tree, WoodenDoor, FarmPlot, WoodenTorch,
	Water, WaterWell, PotatoPlant, OakTree, Potato, Flower1, Flower2
}

var list: Dictionary = {}

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
				var item: Item
				if data is Item:
					item = data
					if list.has(item.item_id):
						push_error("Item Id duplicate found: ", item.item_id, file_name)
					list[item.item_id] = item
			file_name = data_dir.get_next()

func get_by_id(id: Id) -> Item:
	if not list.has(id):
		push_error("No item found with id %s, corresponding to: %s" % [id, Id.keys()[id]])
	return list[id] as Item

func get_constructable_item_ids() -> Array[Id]:
	var result: Array[Id] = []
	for id in list.keys() as Array[Id]:
		var data := get_by_id(id)
		var has_constructable := false
		for component in data.components:
			if component is ConstructableComponent:
				has_constructable = true
				break
		
		if has_constructable:
			result.push_back(id)
	
	return result

func get_crafting_requirements(item_id: Id) -> Array[ItemRequirement]:
	var item := get_by_id(item_id)
	return item.crafting_requirements

func get_item_render_scene(item: Item) -> Node3D:
	var results := item.components.filter(func(component: Component) -> bool:
		return component.id == Components.Id.Scene
		)
	
	if results.size() > 0:
		var scene_component: SceneComponent = results[0]
		var scene := scene_component.scene.instantiate()
		return scene
	
	return null
