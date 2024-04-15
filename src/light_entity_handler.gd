extends Node3D

var entities: Array[EntityLight]

var _processable_entities: Array[EntityLight]

func _ready() -> void:
	Events.request_entity_light_add.connect(func(entity: EntityLight) -> void:
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
		)

func _process(delta: float) -> void:
	for entity: EntityLight in _processable_entities:
		entity.component_container._physics_process(delta)
