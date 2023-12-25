extends Node

enum Id {
	Stone, Wood, WoodenWall
}

var list := {
	Id.Stone: preload("res://items/item_data/stone.tres"),
	Id.Wood: preload("res://items/item_data/wood.tres"),
	Id.WoodenWall: preload("res://items/item_data/wooden_wall.tres"),
}

func get_by_id(id: Id) -> Item:
	return list[id]

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
