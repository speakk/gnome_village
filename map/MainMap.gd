extends TileMap

class_name MainMap

@onready var BLUEPRINT := preload("res://blueprint/Blueprint.tscn")
@onready var HOVER_RECT := preload("res://map/hover_rect.tscn")

const MAP_SIZE_X: int = 80
const MAP_SIZE_Y: int = 40
const CELL_SIZE := Vector2i(24, 24)

var construction_item: Variant # Item | null

enum Layers {
	Ground, Building, Materials, Blueprint
}
#
#const Layers.Ground = 0
#const Layers.Building = 1
#const MATERIALS_LAYER = 2

var map_entities := {
	Layers.Building: [] as Array[Node2D],
	Layers.Materials: [] as Array[Node2D]
}

func _ready() -> void:
	add_layer(Layers.Ground)
	add_layer(Layers.Building)
	add_layer(Layers.Materials)
	add_layer(Layers.Blueprint)

	%HoverRect.visible = false
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(Layers.Ground, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	# TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
	#set_layer_modulate(Layers.Building, Color.TRANSPARENT)
	#set_layer_modulate(Layers.Ground, Color(0.7, 0.7, 0.7))
	set_layer_modulate(Layers.Blueprint, Color(0.5, 0.5, 1.0, 0.5))
	
	Events.blueprint_finished.connect(_blueprint_finished)
	Events.terrain_placed.connect(_terrain_placed)
	
	Events.construction_selected.connect(_construction_selected)
	
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

func set_line_start(coordinate: Vector2i) -> void:
	line_start = coordinate
	
func set_line_end(coordinate: Vector2i) -> void:
	line_end = coordinate

func _process(delta: float) -> void:
	%HoverRect.visible = false
	
	# TODO: Custom draw this stuff
	for hover_rect in $LineHoverRects.get_children():
		hover_rect.queue_free()
	
	if not Input.is_action_pressed("line_draw_modifier"):
		line_start = null
		line_end = null
	
	if construction_item:
		var tile_position: Vector2i = local_to_map(get_local_mouse_position())
		if PathFinder.is_valid_position(tile_position):
			%HoverRect.position = map_to_local(tile_position) - Vector2(CELL_SIZE / 2)
			%HoverRect.visible = true
			
			if is_mouse_pressed:
				if Input.is_action_pressed("line_draw_modifier"):
					if not line_start:
						set_line_start(tile_position)
					
					set_line_end(tile_position)
					var line_coords := get_tile_line(line_start, line_end)
					for line_coord in line_coords:
						var hover_rect := HOVER_RECT.instantiate()
						hover_rect.position = map_to_local(line_coord) - Vector2(CELL_SIZE/2)
						$LineHoverRects.add_child(hover_rect)
				else:
					_place_blueprint(tile_position)
			
			else:
				if line_start and line_end:
					var line_coords := get_tile_line(line_start, line_end)
					for line_coord in line_coords:
						_place_blueprint(line_coord)
				

func _place_blueprint(tile_position: Vector2i) -> void:
	var source_id := get_cell_source_id(Layers.Blueprint, tile_position)
				
	if not PathFinder.is_position_solid(tile_position) and source_id < 0:
		set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
		
		var blueprint := (BLUEPRINT.instantiate() as Blueprint).initialize(Items.Id.WoodenWall)
		blueprint.global_position = coordinate_to_global_position(tile_position)
		get_tree().root.get_node("Main").add_child(blueprint)
		
		Events.blueprint_placed.emit(tile_position, blueprint)
var is_mouse_pressed := false

# TODO: You could use an Area2D and the input_event in that to handle this instead
func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			is_mouse_pressed = true
		else:
			is_mouse_pressed = false
		
			

func coordinate_to_global_position(coordinate: Vector2i) -> Vector2:
	return to_global(map_to_local(coordinate))

func global_position_to_coordinate(_global_position: Vector2) -> Vector2i:
	return local_to_map(to_local(_global_position))

func _terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool) -> void:
	set_cells_terrain_connect(target_layer, [coordinate], terrain_set_id, terrain_id)

# TODO: Also _blueprint_removed
func _blueprint_finished(blueprint: Blueprint) -> void:
	var tile_position := global_position_to_coordinate(blueprint.global_position)
	set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	##set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
	#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	#set_cells_terrain_connect(Layers.Building, [tile_position], 0, 0)
	##set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(1, 0))

func _construction_selected(item: Item) -> void:
	construction_item = item
