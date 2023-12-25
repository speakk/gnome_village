extends TileMap

class_name MainMap

@onready var BLUEPRINT := preload("res://blueprint/Blueprint.tscn")

const MAP_SIZE_X: int = 15
const MAP_SIZE_Y: int = 15
const CELL_SIZE := Vector2i(24, 24)


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
	
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(Layers.Ground, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	# TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
	#set_layer_modulate(Layers.Building, Color.TRANSPARENT)
	set_layer_modulate(Layers.Ground, Color(0.7, 0.7, 0.7))
	set_layer_modulate(Layers.Blueprint, Color(0.5, 0.5, 1.0, 0.5))
	
	Events.blueprint_finished.connect(_blueprint_finished)
	
	Events.map_ready.emit(self)

func _process(delta: float) -> void:
	if is_mouse_pressed:
		var tile_position: Vector2i = local_to_map(get_local_mouse_position())
		var source_id := get_cell_source_id(Layers.Building, tile_position)
		var source_id2 := get_cell_source_id(Layers.Blueprint, tile_position)
		
	# 	TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
		if source_id < 0 and source_id2 < 0:
			#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(1, 0))
			set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
			
			#var blueprint := Blueprint.new().initialize(BuildingTypes.BuildingType.Wall)
			var blueprint := (BLUEPRINT.instantiate() as Blueprint).initialize(BuildingTypes.BuildingType.Wall)
			blueprint.global_position = map_to_local(tile_position)
			#%Entities.add_child(blueprint)
			get_tree().root.get_node("Main").add_child(blueprint)
			
			Events.blueprint_placed.emit(tile_position, blueprint)
			#get_tree().root.get_viewport().set_input_as_handled()

var is_mouse_pressed := false

# TODO: You could use an Area2D and the input_event in that to handle this instead
func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			is_mouse_pressed = true
		else:
			is_mouse_pressed = false
		
			

func _blueprint_finished(blueprint: Blueprint) -> void:
	var tile_position := local_to_map(blueprint.global_position)
	#set_cells_terrain_connect(Layers.Blueprint, [tile_position], 0, 0)
	set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(-1, -1))
	set_cells_terrain_connect(Layers.Building, [tile_position], 0, 0)
	#set_cell(Layers.Blueprint, tile_position, tile_set.get_source_id(1), Vector2i(1, 0))
