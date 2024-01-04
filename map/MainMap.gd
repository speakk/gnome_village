extends TileMap

class_name MainMap

@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")
@onready var HOVER_RECT := preload("res://map/hover_rect.tscn")

const MAP_SIZE_X: int = 80
const MAP_SIZE_Y: int = 40
const CELL_SIZE := Vector2i(24, 24)

var construction_item_id: Variant # Items.Id | null

enum MapActions {
	Build, Dismantle, None
}

var current_action: MapActions = MapActions.None

enum Layers {
	Ground, Building, Materials, Blueprint, Items
}
#
#const Layers.Ground = 0
#const Layers.Building = 1
#const MATERIALS_LAYER = 2

var map_entities := {
	Layers.Blueprint: {} as Dictionary,
	Layers.Building: {} as Dictionary,
	Layers.Items: {} as Dictionary
}



func _ready() -> void:
	add_layer(Layers.Ground)
	add_layer(Layers.Building)
	add_layer(Layers.Materials)
	add_layer(Layers.Blueprint)

	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(Layers.Ground, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	# TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
	#set_layer_modulate(Layers.Building, Color.TRANSPARENT)
	#set_layer_modulate(Layers.Ground, Color(0.7, 0.7, 0.7))
	set_layer_modulate(Layers.Blueprint, Color(0.5, 0.5, 1.0, 0.5))
	
	Events.terrain_placed.connect(_terrain_placed)
	Events.terrain_cleared.connect(_terrain_cleared)
	
	Events.item_placed_on_ground.connect(func(item: ItemOnGround, item_position: Vector2) -> void:
			var coordinate := global_position_to_coordinate(item_position)
			if not map_entities[Layers.Items].has(coordinate):
				map_entities[Layers.Items][coordinate] = []
			map_entities[Layers.Items][coordinate].append(item)
	)
	
	Events.item_removed_from_ground.connect(func(item: ItemOnGround) -> void:
			var coordinate := global_position_to_coordinate(item.global_position)
			if map_entities[Layers.Items].has(coordinate):
				map_entities[Layers.Items][coordinate].erase(item)
	)
	
	Events.construction_selected.connect(_construction_selected)
	Events.dismantle_selected.connect(func() -> void: current_action = MapActions.Dismantle)
	
	Events.map_ready.emit(self)

#Bresenham's line algorithm
func get_tile_line(from: Vector2i, to: Vector2i) -> Array[Vector2i]:
	var points: Array[Vector2i] = []
	var dx := absi(to.x - from.x)
	var dy := -absi(to.y - from.y)
	var err := dx + dy
	var e2 := 2 * err
	var sx := 1 if from.x < to.x else -1
	var sy := 1 if from.y < to.y else -1
	while true:
		points.append(Vector2i(from.x, from.y))
		if from.x == to.x and from.y == to.y:
			break
		e2 = 2 * err
		if e2 >= dy:
			err += dy
			from.x += sx
		if e2 <= dx:
			err += dx
			from.y += sy
	return points

var line_start: Variant # Vector2i | null
var line_end: Variant # Vector2i | null
var line_coords: Array[Vector2i]

var rect_start: Variant # Vector2i | null
var rect_end: Variant # Vector2i | null
var rect_tile_coords: Array[Vector2i]

func set_line_start(coordinate: Vector2i) -> void:
	line_start = coordinate
	
func set_line_end(coordinate: Vector2i) -> void:
	line_end = coordinate

func _handle_build_action(tile_position: Vector2i) -> void:
	if construction_item_id:
		if PathFinder.is_valid_position(tile_position):
			$HoverRectDraw.set_line_coords([tile_position] as Array[Vector2i])
			
			if is_mouse_pressed:
				if Input.is_action_pressed("line_draw_modifier"):
					if not line_start:
						set_line_start(tile_position)
					
					set_line_end(tile_position)
					$HoverRectDraw.set_line_coords(get_tile_line(line_start, line_end))
				else:
					_place_blueprint(tile_position, construction_item_id)
			else:
				if line_start and line_end:
					var line_coords := get_tile_line(line_start, line_end)
					for line_coord in line_coords:
						_place_blueprint(line_coord, construction_item_id)

func _set_rectangle_selection(rect_start_coordinate: Vector2i, rect_end_coordinate: Vector2i) -> void:
	if not rect_start_coordinate or not rect_end_coordinate:
		$RectangleRectDraw.selection_rectangle = null
		return
		
	var start_position := Vector2(map_to_local(rect_start_coordinate)) - Vector2(CELL_SIZE) / 2
	var end_position := Vector2(map_to_local(rect_end_coordinate))
	var selection_rectangle := Rect2(start_position, (end_position - start_position).snapped(Vector2(CELL_SIZE)))
	$RectangleRectDraw.selection_rectangle = selection_rectangle
	
	var new_rect_selection_coordinates: Array[Vector2i] = []
	for y in rect_end_coordinate.y - rect_start_coordinate.y + 1:
		var real_y := y + rect_start_coordinate.y
		for x in rect_end_coordinate.x - rect_start_coordinate.x + 1:
			var real_x := x + rect_start_coordinate.x
			new_rect_selection_coordinates.append(Vector2i(real_x, real_y))
	
	rect_tile_coords = new_rect_selection_coordinates
	print("Rect tile coords set", rect_tile_coords)

func _dismantle_in_position(tile_position: Vector2i) -> void:
	if map_entities[Layers.Items].has(tile_position):
		var entities := map_entities[Layers.Items][tile_position] as Array
		for entity in entities as Array[Node]:
			entity as ItemOnGround
			if entity.item.can_be_dismantled and not entity.reserved_for_dismantling:
				Events.dismantle_issued.emit(entity)

func _handle_dismantle_action(tile_position: Vector2i) -> void:
	if is_mouse_pressed:
		if Input.is_action_pressed("rectangle_select_modifier"):
			if not rect_start:
				rect_start = tile_position
			
			rect_end = tile_position
			_set_rectangle_selection(rect_start, rect_end)
		else:
			_dismantle_in_position(tile_position)
	else:
		if rect_start and rect_end:
			for tile_coordinate in rect_tile_coords:
				_dismantle_in_position(tile_coordinate)
			
			clear_rectangle_selection()

func _handle_map_action(tile_position: Vector2i) -> void:
	if current_action == MapActions.Build:
		_handle_build_action(tile_position)
	if current_action == MapActions.Dismantle:
		_handle_dismantle_action(tile_position)

func clear_rectangle_selection() -> void:
	rect_start = null
	rect_end = null
	rect_tile_coords = []
	$RectangleRectDraw.selection_rectangle = null

func _process(delta: float) -> void:
	if not Input.is_action_pressed("line_draw_modifier"):
		line_start = null
		line_end = null
		line_coords = []
		$HoverRectDraw.set_line_coords([] as Array[Vector2i])
	
	if not Input.is_action_pressed("rectangle_select_modifier"):
		clear_rectangle_selection()
	
	var tile_position: Vector2i = local_to_map(get_local_mouse_position())
	
	if is_mouse_2_pressed:
		_cancel_blueprint(tile_position)
	
	_handle_map_action(tile_position)

func _cancel_blueprint(tile_position: Vector2i) -> void:
	var source_id := get_cell_source_id(Layers.Blueprint, tile_position)
	if source_id > 0:
		var blueprint := map_entities[Layers.Blueprint][tile_position] as ItemOnGround
		Events.blueprint_cancel_issued.emit(blueprint)
		Events.terrain_cleared.emit(tile_position, Layers.Blueprint, source_id)
		#func _terrain_cleared(coordinate: Vector2i, target_layer: MainMap.Layers, tileset_source_id: int, item_on_ground: ItemOnGround) -> void:
		print("Removing at: ", tile_position)
		#map_entities[Layers.Blueprint].erase(tile_position)
		#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	
func _place_blueprint(tile_position: Vector2i, item_id: Items.Id) -> void:
	var source_id := get_cell_source_id(Layers.Blueprint, tile_position)
	if not PathFinder.is_position_solid(tile_position) and source_id < 0:
		var blueprint := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_id, 1, ItemOnGround.ItemState.Blueprint)
		blueprint.global_position = coordinate_to_global_position(tile_position)
		get_tree().root.get_node("Main").add_child(blueprint)
		
		Events.blueprint_placed.emit(tile_position, blueprint)
		
		#set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)



var is_mouse_pressed := false
var is_mouse_2_pressed := false

# TODO: You could use an Area2D and the input_event in that to handle this instead
func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_index == 1:
				is_mouse_pressed = true
			else:
				is_mouse_2_pressed = true
		else:
			is_mouse_pressed = false
			is_mouse_2_pressed = false
		
			

func coordinate_to_global_position(coordinate: Vector2i) -> Vector2:
	return to_global(map_to_local(coordinate))

func global_position_to_coordinate(_global_position: Vector2) -> Vector2i:
	return local_to_map(to_local(_global_position))

func _terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool, item_on_ground: ItemOnGround) -> void:
	set_cells_terrain_connect(target_layer, [coordinate], terrain_set_id, terrain_id)
	map_entities[target_layer][coordinate] = item_on_ground
	#Events.solid_cell_placed.emit(coordinate)
	#set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)

func _terrain_cleared(coordinate: Vector2i, target_layer: MainMap.Layers, tileset_source_id: int) -> void:
	set_cell(target_layer, coordinate, tile_set.get_source_id(tileset_source_id), Vector2i(-1, -1))
	map_entities[target_layer].erase(coordinate) 
	#Events.solid_cell_removed.emit(coordinate)

## TODO: Also _blueprint_removed
#func _construction_finished(blueprint: ItemOnGround) -> void:
	#var tile_position := global_position_to_coordinate(blueprint.global_position)
	#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	#map_entities[Layers.Blueprint].erase(tile_position) 

func _construction_selected(item_id: Items.Id) -> void:
	current_action = MapActions.Build
	construction_item_id = item_id
