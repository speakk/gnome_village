extends TileMap

class_name MainMap

@onready var BLUEPRINT := preload("res://blueprint/Blueprint.tscn")

const MAP_SIZE_X: int = 120
const MAP_SIZE_Y: int = 90

const GROUND_PLAYER = 0
const BUILDING_LAYER = 1

var map_entities := {
	BUILDING_LAYER: [] as Array[Node2D]
}

func _ready() -> void:
	add_layer(GROUND_PLAYER)
	add_layer(BUILDING_LAYER)
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(GROUND_PLAYER, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	# TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
	set_layer_modulate(BUILDING_LAYER, Color.TRANSPARENT)
	
	Events.map_ready.emit(self)

func _process(delta: float) -> void:
	if Input.is_action_pressed("map_interact_a"):
		var tile_position: Vector2i = local_to_map(get_local_mouse_position())
		var tile_data: TileData = get_cell_tile_data(BUILDING_LAYER, tile_position)
		
	# 	TODO: Instead of this, keep a proper x-y map of entities so you don't have to rely on tile_data
		if not tile_data:
			set_cell(BUILDING_LAYER, tile_position, tile_set.get_source_id(0), Vector2i(1, 0))
			
						
			#var blueprint := Blueprint.new().initialize(BuildingTypes.BuildingType.Wall)
			var blueprint := (BLUEPRINT.instantiate() as Blueprint).initialize(BuildingTypes.BuildingType.Wall)
			blueprint.global_position = map_to_local(tile_position)
			#%Entities.add_child(blueprint)
			get_tree().root.get_node("Main").add_child(blueprint)
			
			Events.blueprint_placed.emit(tile_position, blueprint)
