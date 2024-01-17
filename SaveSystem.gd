extends Node

var last_save_id: int = -1

class LoadReference:
	var entity: Variant
	var property_name: String
	var reference_save_id: int

var load_references: Array[LoadReference] = []

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("quicksave"):
		save_state()
	
	if Input.is_action_just_pressed("quickload"):
		load_state()

func enrich_save_data(entity: Variant, entity_dict: Dictionary) -> void:
	entity_dict["save_id"] = entity.persistent.get_save_id()
	entity_dict["parent"] = entity.get_parent().get_path()
	entity_dict["filename"] = entity.get_scene_file_path()

func get_next_save_id() -> int:
	last_save_id += 1
	return last_save_id

func load_state() -> void:
	load_references = []
	
	var save_game := FileAccess.open("user://savegame.save", FileAccess.READ)
	var save_dict := JSON.parse_string(save_game.get_line()) as Dictionary
	
	last_save_id = save_dict["last_save_id"]
	
	Events.load_game_called.emit(save_dict)
	
	await get_tree().physics_frame
	
	#var main := get_node("/root/Main") as Main
	#main.load_save(save_dict["main_data"])
	
	# TODO: All "entities" stuff should probably be in Main eventually
	fill_in_references(save_dict["main_data"]["entities"])

func save_state() -> void:
	var save_game := FileAccess.open("user://savegame.save", FileAccess.WRITE)
	var save_dict: Dictionary = {}
	save_dict["last_save_id"] = last_save_id
	
	#var main := get_node("/root/Main") as Main
	#var main_dict := main.save()
	
	Events.save_game_called.emit(save_dict)
	
	await get_tree().physics_frame
	
	#save_dict["main_data"] = main_dict

	save_game.store_line(JSON.stringify(save_dict))

#SaveSystem.register_load_reference(self, "current_task", save_dict["current_task_save_id"])

func register_load_reference(entity: Variant, property_name: String, reference_save_id: int) -> void:
	var new_reference := LoadReference.new()
	new_reference.entity = entity
	new_reference.property_name = property_name
	new_reference.reference_save_id = reference_save_id
	
	load_references.append(new_reference)

func fill_in_references(entities_dict: Dictionary) -> void:
	for reference in load_references:
		reference.entity[reference.property_name] = entities_dict[reference.reference_save_id]
