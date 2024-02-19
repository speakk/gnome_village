extends PanelContainer

@onready var ENTITY_INFO_DISPLAY := preload("res://src/gui/entity_info/entity_info_display.tscn")

var selected_entities: Array[Node3D]

func _ready() -> void:
	Events.entity_selected.connect(_entity_selected)
	Events.entity_deselected.connect(_entity_deselected)
	Events.clear_entity_selections.connect(_clear_entity_selections)
	redraw()

func _entity_selected(entity: Node3D) -> void:
	if not selected_entities.has(entity):
		selected_entities.append(entity)
		redraw()

func _entity_deselected(entity: Node3D) -> void:
	if selected_entities.has(entity):
		selected_entities.erase(entity)
		redraw()

func _clear_entity_selections() -> void:
	selected_entities.clear()
	redraw()

func redraw() -> void:
	for child in %EntityInfoDisplays.get_children():
		child.queue_free()
	
	for entity in selected_entities:
		var entity_info_display := ENTITY_INFO_DISPLAY.instantiate()
		%EntityInfoDisplays.add_child(entity_info_display)
		create_tween().tween_property($PanelContainer, "position", Vector2(entity_info_display.get_rect().size.x, 0), 0.0)
		create_tween().tween_property($PanelContainer, "position", Vector2(), 0.3).set_delay(0.01).set_trans(Tween.TRANS_SPRING).set_ease(Tween.EASE_OUT)
		entity_info_display.set_entity(entity)

