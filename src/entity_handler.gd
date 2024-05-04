class_name EntityHandler extends Node3D

var entities: Array[Entity]
var _processable_entities: Array[Entity]

# Map item type to Array[Entity]
var item_amount_map: Dictionary
var all_items_with_amounts: Array[Entity]

class ItemLocationEntry:
	var item_amount: ItemAmountComponent
	var entity: Entity
	var coordinate: Vector2i

func _ready() -> void:
	add_to_group("entity_handler")
	Events.request_entity_add.connect(add_entity)
	Events.component.item_amount_changed.connect(_item_amount_changed)

#func _item_amount_changed(item_amount: ItemAmountComponent) -> void:
	#var entity_definition := item_amount.item
	#if not item_amount_map.has(entity_definition):
		#item_amount_map[entity_definition] = []
	#
	#var list_of_items: Array = item_amount_map.get(entity_definition)
#
	#if list_of_items.has(item_amount):
		#if item_amount.amount == 0:
			#list_of_items.erase(item_amount.get_owner())
		#return
	#else:
		#list_of_items.append(item_amount.get_owner())
	#
	#Events.entities.item_amounts_changed.emit(item_amount_map.values())

func _item_amount_changed(item_amount: ItemAmountComponent) -> void:
	var entity_definition := item_amount.item
	var item_owner := item_amount.get_owner()
	if item_amount.amount == 0:
		all_items_with_amounts.erase(item_owner)
	else:
		if not all_items_with_amounts.has(item_owner):
			all_items_with_amounts.append(item_owner)
	
	Events.entities.item_amounts_changed.emit(all_items_with_amounts)

func get_items_of_type(entity_definition: EntityDefinition) -> Array[ItemLocationEntry]:
	var result: Array[ItemLocationEntry] = []
	result.assign(item_amount_map.get(entity_definition))
	return result

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
	var entity_dicts: Array[Dictionary] = []
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
	
