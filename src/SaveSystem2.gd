extends Node

var current_save_id: int = 0
var save_methods: Array[SaveMethod]
var entity_references: Dictionary
var entity_reference_queue: Array[EntityReferenceEntry]

class EntityReferenceEntry:
	var save_id: int
	var callable: Callable
	func _init(_save_id: int, _callable: Callable) -> void:
		save_id = _save_id
		callable = _callable

class SaveMethod:
	var dict_key: String
	var save_callable: Callable
	var load_callable: Callable
	
	func _init(_dict_key: String, _save_callable: Callable, _load_callable: Callable) -> void:
		dict_key = _dict_key
		save_callable = _save_callable
		load_callable = _load_callable


func register_save_method(save_method: SaveMethod) -> void:
	for existing_method in save_methods:
		if existing_method.dict_key == save_method.dict_key:
			save_methods.erase(existing_method)
			break
			
	save_methods.append(save_method)

func save_game() -> void:
	var save_dict: Dictionary
	
	for save_method in save_methods:
		save_dict[save_method.dict_key] = save_method.save_callable.call()

	var save_file := FileAccess.open("user://savegame.save", FileAccess.WRITE)
	save_file.store_line(var_to_str(save_dict))

func load_game(save_name: String) -> void:
	var save_file := FileAccess.open("user://%s.save" % save_name, FileAccess.READ)
	var dict: Dictionary = str_to_var(save_file.get_as_text())
	
	for save_method in save_methods:
		save_method.load_callable.call(dict[save_method.dict_key])
	
	for entity_reference_entry in entity_reference_queue:
		entity_reference_entry.callable.call(entity_references[entity_reference_entry.save_id])

func quick_load() -> void:
	load_game("savegame")

func _get_next_save_id() -> int:
	current_save_id += 1
	return current_save_id

func get_save_id(object: Variant) -> int:
	if not object.has_meta("save_id"):
		object.set_meta("save_id", _get_next_save_id())
	return object.get_meta("save_id")
	
func register_entity_reference(object: Variant) -> void:
	entity_references[object.get_meta("save_id")] = object

func queue_entity_reference_by_id(entity_reference_entry: EntityReferenceEntry) -> void:
	entity_reference_queue.append(entity_reference_entry)