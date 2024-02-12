extends Node

enum Id {
	Potato
}

var plant_resources: Dictionary = {
	Id.Potato: preload("res://src/plants/plant_data/potato_plant.tres")
}

func get_plant_by_id(plant_id: Id) -> Plant:
	return plant_resources.get(plant_id)
