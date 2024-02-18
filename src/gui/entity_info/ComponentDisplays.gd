extends Node

var component_to_display_map: Dictionary = {
	Components.Id.DisplayName: preload("res://src/gui/entity_info/component_display/NameDisplay.tscn"),
	Components.Id.CharacterStats: preload("res://src/gui/entity_info/component_display/CharacterStatsDisplay.tscn"),
}

func get_component_display(component_id: Components.Id) -> Control:
	if component_to_display_map.has(component_id):
		return component_to_display_map.get(component_id).instantiate()
	
	return null
