extends TileMap

class_name MainMap

@onready var BLUEPRINT := preload("res://blueprint/Blueprint.tscn")

const MAP_SIZE_X: int = 60
const MAP_SIZE_Y: int = 40

enum Layers {
	Ground, Building, Materials
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
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(Layers.Ground, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	# TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
	set_layer_modulate(Layers.Building, Color.TRANSPARENT)
	set_layer_modulate(Layers.Ground, Color(0.7, 0.7, 0.7))
	
	Events.map_ready.emit(self)

func _process(delta: float) -> void:
	if Input.is_action_pressed("map_interact_a"):
		var tile_position: Vector2i = local_to_map(get_local_mouse_position())
		var tile_data: TileData = get_cell_tile_data(Layers.Building, tile_position)
		
	# 	TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
		if not tile_data:
			set_cell(Layers.Building, tile_position, tile_set.get_source_id(0), Vector2i(1, 0))
			
						
			#var blueprint := Blueprint.new().initialize(BuildingTypes.BuildingType.Wall)
			var blueprint := (BLUEPRINT.instantiate() as Blueprint).initialize(BuildingTypes.BuildingType.Wall)
			blueprint.global_position = map_to_local(tile_position)
			#%Entities.add_child(blueprint)
			get_tree().root.get_node("Main").add_child(blueprint)
			
			Events.blueprint_placed.emit(tile_position, blueprint)
