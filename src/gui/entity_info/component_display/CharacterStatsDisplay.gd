extends MarginContainer

#@onready var CHARACTER_STATE_VALUE_DISPLAY := preload("res://src/gui/entity_info/component_display/character_stat_value_display.tscn")

func set_component(component: Component) -> void:
	component = component as CharacterStatsComponent
	for child in %Stats.get_children():
		child.queue_free()
	
	print("STATS LENGTH: ", component.get_stats().size())
	for stat: CharacterStatsComponent.CharacterStat in component.get_stats():
		var label := Label.new()
		label.text = stat.display_name
		label.custom_minimum_size.x = 150
		
		print("Loading valuedisplays: ", stat.display_name)
		var value_display: CharacterStatValueDisplay = load("res://src/gui/entity_info/component_display/character_stat_value_display.tscn").instantiate()
		value_display.set_stat(stat)
#A
		var hbox := HBoxContainer.new()
		hbox.add_child(label)
		hbox.add_child(value_display)
		
		%Stats.add_child(hbox)
