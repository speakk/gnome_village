extends Node

enum Id {
	Stone, Wood, WoodenWall, Tree, WoodenDoor
}

var list: Dictionary = {
	Id.Stone: load("res://items/item_data/stone.tres") as Item,
	Id.Wood: load("res://items/item_data/wood.tres") as Item,
	Id.WoodenWall: load("res://items/item_data/wooden_wall.tres") as Item,
	Id.Tree: load("res://items/item_data/tree.tres") as Item,
	Id.WoodenDoor: load("res://items/item_data/wooden_door.tres") as Item,
}

func get_by_id(id: Id) -> Item:
	return list[id] as Item

func get_constructable_item_ids() -> Array[Id]:
	var result: Array[Id] = []
	for id in list.keys() as Array[Id]:
		var data := get_by_id(id)
		if data.can_be_constructed:
			result.push_back(id)
	
	return result

func get_crafting_requirements(item_id: Id) -> Array[ItemRequirement]:
	var item := get_by_id(item_id)
	return item.crafting_requirements
