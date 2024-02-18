extends PanelContainer

@onready var ENTITY_INFO_DISPLAY := preload("res://src/gui/entity_info/entity_info_display.tscn")

var selected_entities: Array[Node3D]

func _ready() -> void:
	Events.entity_selected.connect(_entity_selected)
	Events.entity_deselected.connect(_entity_deselected)
	redraw()

func _entity_selected(entity: Node3D) -> void:
	if not selected_entities.has(entity):
		selected_entities.append(entity)
		redraw()

func _entity_deselected(entity: Node3D) -> void:
	if selected_entities.has(entity):
		selected_entities.erase(entity)
		redraw()

func redraw() -> void:
	if selected_entities.size() == 0:
		$EntityInfoDisplay.hide()
	else:
		$EntityInfoDisplay.show()
	
	for child in %EntityInfoDisplays.get_children():
		child.queue_free()
	
	for entity in selected_entities:
		var entity_info_display := ENTITY_INFO_DISPLAY.instantiate()
		%EntityInfoDisplays.add_child(entity_info_display)
		entity_info_display.set_entity(entity)

