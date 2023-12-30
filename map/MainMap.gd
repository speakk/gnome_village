extends TileMap

class_name MainMap

@onready var BLUEPRINT := preload("res://blueprint/Blueprint.tscn")
@onready var HOVER_RECT := preload("res://map/hover_rect.tscn")

const MAP_SIZE_X: int = 80
const MAP_SIZE_Y: int = 40
const CELL_SIZE := Vector2i(24, 24)

var construction_item: Variant # Item | null

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
	
	Events.blueprint_finished.connect(_blueprint_finished)
	Events.terrain_placed.connect(_terrain_placed)
	
	Events.item_placed_on_ground.connect(func(item: ItemOnGround, item_position: Vector2) -> void:
			var coordinate := global_position_to_coordinate(item_position)
			if not map_entities[Layers.Items].has(coordinate):
				map_entities[Layers.Items][coordinate] = []
			map_entities[Layers.Items][coordinate].append(item)
	)
	
	Events.item_removed_from_ground.connect(func(item: ItemOnGround, item_position: Vector2) -> void:
			var coordinate := global_position_to_coordinate(item_position)
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

func set_line_start(coordinate: Vector2i) -> void:
	line_start = coordinate
	
func set_line_end(coordinate: Vector2i) -> void:
	line_end = coordinate

func _handle_build_action(tile_position: Vector2i) -> void:
	if construction_item:
		if PathFinder.is_valid_position(tile_position):
			$HoverRectDraw.set_line_coords([tile_position] as Array[Vector2i])
			
			if is_mouse_pressed:
				if Input.is_action_pressed("line_draw_modifier"):
					if not line_start:
						set_line_start(tile_position)
					
					set_line_end(tile_position)
					$HoverRectDraw.set_line_coords(get_tile_line(line_start, line_end))
				else:
					_place_blueprint(tile_position)
			else:
				if line_start and line_end:
					var line_coords := get_tile_line(line_start, line_end)
					for line_coord in line_coords:
						_place_blueprint(line_coord)

func _handle_dismantle_action(tile_position: Vector2i) -> void:
	if is_mouse_pressed:
		print("Handle dismantle action mouse pressed")
		if map_entities[Layers.Items].has(tile_position):
			var entities := map_entities[Layers.Items][tile_position] as Array
			for entity in entities as Array[Node]:
				entity as ItemOnGround
				if entity.item.can_be_dismantled:
					Events.dismantle_issued.emit(entity)
					print("Issued")
			#var entity 
			#Events.dismantle_issued

func _handle_map_action(tile_position: Vector2i) -> void:
	if current_action == MapActions.Build:
		_handle_build_action(tile_position)
	if current_action == MapActions.Dismantle:
		_handle_dismantle_action(tile_position)
	

func _process(delta: float) -> void:
	if not Input.is_action_pressed("line_draw_modifier"):
		line_start = null
		line_end = null
		line_coords = []
		$HoverRectDraw.set_line_coords([] as Array[Vector2i])
	
	var tile_position: Vector2i = local_to_map(get_local_mouse_position())
	
	if is_mouse_2_pressed:
		_cancel_blueprint(tile_position)
	
	_handle_map_action(tile_position)
	

func _cancel_blueprint(tile_position: Vector2i) -> void:
	var source_id := get_cell_source_id(Layers.Blueprint, tile_position)
	if source_id > 0:
		print("Removing at: ", tile_position)
		var blueprint := map_entities[Layers.Blueprint][tile_position] as Blueprint
		Events.blueprint_cancel_issued.emit(blueprint)
		map_entities[Layers.Blueprint].erase(tile_position)
		set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	

func _place_blueprint(tile_position: Vector2i) -> void:
	var source_id := get_cell_source_id(Layers.Blueprint, tile_position)
				
	if not PathFinder.is_position_solid(tile_position) and source_id < 0:
		set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
		
		var blueprint := (BLUEPRINT.instantiate() as Blueprint).initialize(Items.Id.WoodenWall)
		blueprint.global_position = coordinate_to_global_position(tile_position)
		get_tree().root.get_node("Main").add_child(blueprint)
		
		map_entities[Layers.Blueprint][tile_position] = blueprint
		print("Placed at", tile_position)
		Events.blueprint_placed.emit(tile_position, blueprint)
		
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
						terrain_set_id: int, terrain_id: int, is_solid: bool) -> void:
	set_cells_terrain_connect(target_layer, [coordinate], terrain_set_id, terrain_id)

# TODO: Also _blueprint_removed
func _blueprint_finished(blueprint: Blueprint) -> void:
	var tile_position := global_position_to_coordinate(blueprint.global_position)
	set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	map_entities[Layers.Blueprint].erase(tile_position) 
	##set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
	#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	#set_cells_terrain_connect(Layers.Building, [tile_position], 0, 0)
	##set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(1, 0))

func _construction_selected(item: Item) -> void:
	current_action = MapActions.Build
	construction_item = item
