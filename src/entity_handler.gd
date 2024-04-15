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

func _process(delta: float) -> void:
	for entity: Entity in _processable_entities:
		entity.component_container._physics_process(delta)

func get_all() -> Array[Entity]:
	return entities
