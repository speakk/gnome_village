class_name SelectionDraw extends Node3D

@onready var TILE_INDICATOR := preload("res://src/map/TileIndicator.tscn")

var line_coords: Array[Vector2i]:
	set(new_value):
		line_coords = new_value
		redraw()

var selection_rectangle: Variant:
	set(new_value):
		selection_rectangle = new_value
		redraw()

#
#func _draw() -> void:
	#if line_coords and line_coords.size() > 0:
		#for line_coord in line_coords:
			#var tile_position := Globals.get_map().map_to_local(line_coord)
			#var rect := Rect2(tile_position - Vector2(MainMap.CELL_SIZE) / 2, MainMap.CELL_SIZE)
			#draw_rect(rect, Color.BISQUE, false, 1)
	#
	#if selection_rectangle:
		#draw_rect(selection_rectangle, Color.BISQUE, false, 1)

func redraw() -> void:
	for child in %TileIndicators.get_children():
		child.queue_free()
	
	if line_coords and line_coords.size() > 0:
		for line_coord in line_coords:
			var tile_position := Globals.get_map().grid.map_to_local(Globals.extend_vec2i(line_coord))
			#print("tile position", tile_position)
			var tile_indicator := TILE_INDICATOR.instantiate() as MeshInstance3D
			%TileIndicators.add_child(tile_indicator)
			tile_indicator.global_position = Vector3(tile_position.x, 0, tile_position.z)
#
	#if selection_rectangle:
		#
