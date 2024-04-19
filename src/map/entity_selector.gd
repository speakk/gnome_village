class_name EntitySelector extends Node

@export var main_map: MainMap

var _selected_entites: Array[Entity]

func reset() -> void:
	clear_selections()

func clear_selections() -> void:
	for entity in _selected_entites:
		if entity.component_container.has_component(Components.Id.Selectable):
			entity.component_container.get_by_id(Components.Id.Selectable).selected = false
	
	_selected_entites.clear()

func select_next_entity(coordinates: Array[Vector2i]) -> void:
	if coordinates.size() == 1:
		print("Select next entity")
		var entity_to_select: Entity
		var coordinate := coordinates[0]
		var entities := main_map.get_map_entities(coordinate)
		for entity in entities:
			print("Going though entity", entity)
			if entity.component_container.has_component(Components.Id.Selectable):
				print("Had selectable")
				if not entity.component_container.get_by_id(Components.Id.Selectable).selected:
					entity_to_select = entity
					break
		
		
		if not entity_to_select:
			if entities.size() > 1:
				entity_to_select = entities[0]
			elif entities.size() == 1:
				return
		
		clear_selections()
		
		if entity_to_select:
			print("Setting as selected")
			var selectable: SelectableComponent = entity_to_select.component_container.get_by_id(Components.Id.Selectable)
			if selectable:
				selectable.selected = true
				_selected_entites.append(entity_to_select)
	
	else:
		clear_selections()
		for coordinate in coordinates:
			var entities := main_map.get_map_entities(coordinate)
			for entity in entities:
				if entity.component_container.has_component(Components.Id.Selectable):
					entity.component_container.get_by_id(Components.Id.Selectable).selected = true
					_selected_entites.append(entity)
					
