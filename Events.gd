extends Node

signal blueprint_placed(tile_position: Vector2i, blueprint: ItemOnGround)
signal construction_finished(item: ItemOnGround)
signal blueprint_cancel_issued(blueprint: ItemOnGround)

signal dismantle_issued(item_on_ground: ItemOnGround)
signal dismantle_finished(item_on_ground: ItemOnGround)
signal dismantle_cancel_issued(item_on_ground: ItemOnGround)

signal debug_visuals_set(value: bool)

signal task_finished(task: Task)

signal map_ready(map: MainMap)
signal solid_cell_placed(coordinate: Vector2i)
signal solid_cell_removed(coordinate: Vector2i)

signal terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool, item_on_ground: ItemOnGround)

signal terrain_cleared(coordinate: Vector2i, target_layer: MainMap.Layers, tileset_source_id: int)

signal construction_selected(item_id: Items.Id)
signal dismantle_selected()

signal item_placed_on_ground(item: ItemOnGround, item_position: Vector2)
signal item_removed_from_ground(item: ItemOnGround, item_position: Vector2)
