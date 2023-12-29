extends Node

signal blueprint_placed(tile_position: Vector2i, blueprint: Blueprint)
signal blueprint_finished(blueprint: Blueprint)
signal blueprint_cancel_issued(blueprint: Blueprint)

signal map_ready(map: MainMap)
signal solid_cell_placed(coordinate: Vector2i)
signal solid_cell_removed(coordinate: Vector2i)

signal terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool)

signal construction_selected(item: Item)
