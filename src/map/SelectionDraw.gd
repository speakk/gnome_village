class_name SelectionDraw extends Node3D

@onready var TILE_INDICATOR := preload("res://src/map/TileIndicator.tscn")

var selected_ui_action: UiAction

func _ready() -> void:
	Events.ui_action_selected.connect(func(new_ui_action: UiAction) -> void: selected_ui_action = new_ui_action)

var line_coords: Array[Vector2i]:
	set(new_value):
		line_coords = new_value
		redraw()

var selection_rectangle: Variant:
	set(new_value):
		selection_rectangle = new_value
		redraw()

func redraw() -> void:
	for child in %TileIndicators.get_children():
		child.queue_free()
		
	var shape_component: ShapeComponent = ShapeComponent.new()
	shape_component.shape_definition = "1"
	
	if selected_ui_action is UiAction.Build:
		var entity_shape_component: ShapeComponent = selected_ui_action.item.get_component_by_id(Components.Id.Shape)
		if entity_shape_component:
			shape_component = entity_shape_component
	
	if line_coords and line_coords.size() > 0:
		for line_coord in line_coords:
			for y in shape_component.get_shape().size():
				var shape_row: ShapeRow = shape_component.get_shape()[y]
				for x in shape_row.row.size():
					var value: bool = shape_row.row[x]
					if value:
						var final_coord: Vector2i = line_coord + Vector2i(x, y) - shape_component.origin
						var tile_position := Globals.get_map().grid.map_to_local(Globals.extend_vec2i(final_coord))
						var tile_indicator := TILE_INDICATOR.instantiate() as MeshInstance3D
						%TileIndicators.add_child(tile_indicator)
						tile_indicator.global_position = Vector3(tile_position.x, 0, tile_position.z)
