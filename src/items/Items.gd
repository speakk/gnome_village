extends Node

enum Id {
	Stone, Wood, WoodenWall, Tree, WoodenDoor, FarmPlot, WoodenTorch,
	Water, WaterWell, PotatoPlant
}

var list: Dictionary = {
	Id.Stone: load("res://src/items/item_data/stone.tres") as Item,
	Id.Wood: load("res://src/items/item_data/wood.tres") as Item,
	Id.WoodenWall: load("res://src/items/item_data/wooden_wall.tres") as Item,
	Id.Tree: load("res://src/items/item_data/tree.tres") as Item,
	Id.WoodenDoor: load("res://src/items/item_data/wooden_door.tres") as Item,
	Id.FarmPlot: load("res://src/items/item_data/farm_plot.tres") as Item,
	Id.WoodenTorch: load("res://src/items/item_data/wooden_torch.tres") as Item,
	Id.Water: load("res://src/items/item_data/water.tres") as Item,
	Id.WaterWell: load("res://src/items/item_data/well.tres") as Item,
	Id.PotatoPlant: load("res://src/items/item_data/plants/potato_plant.tres") as Item
}

func get_by_id(id: Id) -> Item:
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
