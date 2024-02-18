extends PanelContainer

@onready var ENTITY_INFO_DISPLAY := preload("res://src/gui/entity_info_display.tscn")

var selected_entities: Array[Node3D]

func _ready() -> void:
	Events.entity_selected.connect(_entity_selected)
	Events.entity_deselected.connect(_entity_deselected)

func _entity_selected(entity: Node3D) -> void:
	selected_entities.append(entity)
	redraw()

func _entity_deselected(entity: Node3D) -> void:
	selected_entities.erase(entity)
	redraw()

func redraw() -> void:
	for child in %EntityInfoDisplays.get_children():
		child.queue_free()
	
	for entity in selected_entities:
		var entity_info_display := ENTITY_INFO_DISPLAY.instantiate()
		add_child(entity_info_display)
		entity_info_display.set_entity(entity)

