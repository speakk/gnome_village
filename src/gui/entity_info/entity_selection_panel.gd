extends PanelContainer

@onready var ENTITY_INFO_DISPLAY := preload("res://src/gui/entity_info/entity_info_display.tscn")

var selected_entities: Array[Entity]

func _ready() -> void:
	Events.entity_selected.connect(_entity_selected)
	Events.entity_deselected.connect(_entity_deselected)
	Events.clear_entity_selections.connect(_clear_entity_selections)
	redraw()

var queue_redraw := false

func _process(delta: float) -> void:
	if queue_redraw:
		redraw()
		queue_redraw = false

func _entity_selected(entity: Entity) -> void:
	if not selected_entities.has(entity):
		selected_entities.append(entity)
		queue_redraw = true

func _entity_deselected(entity: Entity) -> void:
	if selected_entities.has(entity):
		selected_entities.erase(entity)
		queue_redraw = true

func _clear_entity_selections() -> void:
	selected_entities.clear()
	queue_redraw = true

func redraw() -> void:
	var panel_offset: float = 300
	if selected_entities.size() == 0:
		create_tween().tween_property($PanelContainer, "position", Vector2(), 0)
		await create_tween().tween_property($PanelContainer, "position", Vector2(panel_offset, 0), 0.15).set_delay(0.01).set_trans(Tween.TRANS_QUAD).set_ease(Tween.EASE_IN).finished
		for child in %EntityInfoDisplays.get_children():
			child.queue_free()
	else:
		for child in %EntityInfoDisplays.get_children():
			child.queue_free()
		
		for entity in selected_entities:
			var entity_info_display := ENTITY_INFO_DISPLAY.instantiate()
			%EntityInfoDisplays.add_child(entity_info_display)
			entity_info_display.set_entity(entity)
		
		create_tween().tween_property($PanelContainer, "position", Vector2(panel_offset, 0), 0.0)
		create_tween().tween_property($PanelContainer, "position", Vector2(), 0.3).set_delay(0.01).set_trans(Tween.TRANS_SPRING).set_ease(Tween.EASE_OUT)

