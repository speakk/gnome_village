extends TileMap

class_name MainMap

const MAP_SIZE_X: int = 120
const MAP_SIZE_Y: int = 90

const GROUND_PLAYER = 0
const BUILDING_LAYER = 1

func _ready() -> void:
	add_layer(GROUND_PLAYER)
	add_layer(BUILDING_LAYER)
	
	for x in MAP_SIZE_X:
		for y in MAP_SIZE_Y:
			set_cell(GROUND_PLAYER, Vector2i(x, y), tile_set.get_source_id(0), Vector2i(0, 0))
	
	Events.map_ready.emit(self)

func _process(delta: float) -> void:
	if Input.is_action_pressed("map_interact_a"):
		var tile_position: Vector2i = local_to_map(get_local_mouse_position())
		var tile_data: TileData = get_cell_tile_data(BUILDING_LAYER, tile_position)
		
		if not tile_data:
			set_cell(BUILDING_LAYER, tile_position, tile_set.get_source_id(0), Vector2i(1, 0))
			Events.BlueprintPlaced.emit(tile_position, BuildingTypes.BuildingType.Wall)
