class_name SaveSystem extends Node

@export var map: MainMap
@export var entity_container: Node

func save_game() -> void:
	var entities := get_tree().get_nodes_in_group("entity")
	var entity_dicts: Array[Dictionary]
	for entity: Entity in entities as Array[Entity]:
		var save_dict := entity.serialize()
		entity_dicts.append(save_dict)
	
	var saved_map := map.serialize()
	var task_manager: Dictionary = TaskManager.serialize()
	
	var save_dict := {
		entities = entity_dicts,
		map = saved_map,
		task_manager = task_manager
	}

	var save_file := FileAccess.open("user://savegame.save", FileAccess.WRITE)
	save_file.store_line(var_to_str(save_dict))

func load_game(save_name: String) -> void:
	var save_file := FileAccess.open("user://%s.save" % save_name, FileAccess.READ)
	var dict: Dictionary = str_to_var(save_file.get_as_text())
	
	for entity_dict: Dictionary in dict["entities"]:
		var entity: Entity = Entity.deserialize(entity_container, entity_dict)

func quick_load() -> void:
	load_game("savegame")
