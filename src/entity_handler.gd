class_name EntityHandler extends Node3D

var entities: Array[Entity]

var _processable_entities: Array[Entity]

func _ready() -> void:
	add_to_group("entity_handler")
	Events.request_entity_add.connect(add_entity)

func add_entity(entity: Entity) -> void:
	entities.append(entity)
	entity.on_enter()
	entity.component_container.is_processable.connect(func() -> void:
		if not _processable_entities.has(entity):
			_processable_entities.append(entity)
		)

	entity.component_container.is_not_processable.connect(func() -> void:
		if _processable_entities.has(entity):
			_processable_entities.erase(entity)
		)
	
	entity.delete_called.connect(func() -> void:
		entities.erase(entity)
		_processable_entities.erase(entity)
		)

func _process(delta: float) -> void:
	for entity: Entity in _processable_entities:
		entity.component_container._physics_process(delta)

func get_all() -> Array[Entity]:
	return entities

func reset() -> void:
	entities.clear()
	_processable_entities.clear()

func serialize() -> Dictionary:
	var entity_dicts: Array[Dictionary]
	for entity: Entity in get_all():
		var save_dict := entity.serialize()
		entity_dicts.append(save_dict)
	
	return {
		entities = entity_dicts
	}

func deserialize(save_dict: Dictionary) -> void:
	for entity_dict: Dictionary in save_dict["entities"]:
		var entity := Entity.static_deserialize(entity_dict)
		add_entity(entity)
	
