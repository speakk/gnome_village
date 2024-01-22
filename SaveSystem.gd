extends Node

var last_save_id: int = -1

class LoadReference:
	var entity: Variant
	var property_name: String
	var reference_save_id: int
	var add_as_child: bool

var load_references: Array[LoadReference] = []

var saved_entities: Dictionary
var loaded_entities: Dictionary
var entity_dicts: Dictionary

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("quicksave"):
		save_state()
	
	if Input.is_action_just_pressed("quickload"):
		load_state()

func enrich_save_data(entity: Variant, entity_dict: Dictionary) -> void:
	entity_dict["save_id"] = get_object_save_id(entity)
	#entity_dict["parent"] = entity.get_parent().get_path()
	if not entity is Resource:
		var file_path: String = entity.get_scene_file_path()
		entity_dict["filename"] = file_path
		assert(file_path)
	else:
		#var classname := entity.get_class_name() as String
		var resource_path := entity.get_path() as String
		entity_dict["resource_path"] = resource_path

func get_next_save_id() -> int:
	last_save_id += 1
	return last_save_id

func load_state() -> void:
	load_references = []
	
	var save_game := FileAccess.open("user://savegame.save", FileAccess.READ)
	var save_dict := JSON.parse_string(save_game.get_line()) as Dictionary
	
	last_save_id = save_dict["last_save_id"]
	entity_dicts = save_dict["entities"]
	
	for entity_dict in save_dict["entities"].values() as Array[Dictionary]:
		var new_object: Variant
		if entity_dict.has("resource_path"):
			new_object = ResourceLoader.load(entity_dict.get("resource_path"))
			#new_object = ClassDB.instantiate(entity_dict.get("class_name"))
		else:
			new_object = load(entity_dict["filename"]).instantiate()
			
		#new_object.call_deferred("load_save", entity_dict)
		new_object.set_meta("save_id", entity_dict["save_id"] as int)
		loaded_entities[entity_dict["save_id"] as int] = new_object
	
	Events.load_game_called.emit(save_dict)
	
	await get_tree().physics_frame
	
	load_entity_saves(save_dict["entities"].values())
	
	await get_tree().physics_frame
	
	#var main := get_node("/root/Main") as Main
	#main.load_save(save_dict["main_data"])
	
	# TODO: All "entities" stuff should probably be in Main eventually
	fill_in_references(save_dict["main_data"]["entities"])
	fill_in_references(save_dict["main_data"]["tasks"])

func load_entity_saves(entities_orig: Array) -> void:
	var entities: Array[Dictionary]
	entities.assign(entities_orig)
	for entity_dict in entities:
		loaded_entities[entity_dict["save_id"] as int].load_save(entity_dict)

func save_state() -> void:
	saved_entities = {}
	var save_game := FileAccess.open("user://savegame.save", FileAccess.WRITE)
	var save_dict: Dictionary = {}
	save_dict["last_save_id"] = last_save_id
	
	#var main := get_node("/root/Main") as Main
	#var main_dict := main.save()
	
	Events.save_game_called.emit(save_dict)
	
	await get_tree().physics_frame
	
	#save_dict["main_data"] = main_dict
	
	save_dict["entities"] = saved_entities

	save_game.store_line(JSON.stringify(save_dict))

#SaveSystem.register_load_reference(self, "current_task", save_dict["current_task_save_id"])

func register_load_reference(entity: Variant, property_name: String, reference_save_id: int, add_as_child: bool = false) -> void:
	var new_reference := LoadReference.new()
	new_reference.entity = entity
	new_reference.property_name = property_name
	new_reference.reference_save_id = reference_save_id
	new_reference.add_as_child = add_as_child
	
	load_references.append(new_reference)

func get_object_save_id(entity: Variant) -> int:
	if entity.has_meta("save_id"):
		return entity.get_meta("save_id")
		
	var save_id := SaveSystem.get_next_save_id()
	entity.set_meta("save_id", save_id)
	
	return save_id

func save_entity(entity: Variant) -> int:
	var save_dict := entity.save() as Dictionary
	enrich_save_data(entity, save_dict)
	saved_entities[get_object_save_id(entity)] = save_dict
	return get_object_save_id(entity)

func load_entity(entity: Variant) -> void:
	var save_dict: Dictionary = entity_dicts["%s" % get_object_save_id(entity)]
	entity.load_save(save_dict)

func get_saved_entity(entity_id: int) -> Variant:
	return loaded_entities[entity_id]

func fill_in_references(entity_ids: Array) -> void:
	for reference in load_references:
		var referenced_entity: Variant = loaded_entities[reference.reference_save_id]
		reference.entity[reference.property_name] = referenced_entity
		if reference.add_as_child:
			reference.entity.add_child(referenced_entity)
