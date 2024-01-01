extends Node

signal blueprint_placed(tile_position: Vector2i, blueprint: Blueprint)
signal blueprint_finished(blueprint: Blueprint)
signal blueprint_cancel_issued(blueprint: Blueprint)

signal dismantle_issued(item_on_ground: ItemOnGround)
signal dismantle_finished(item_on_ground: ItemOnGround)
signal dismantle_cancel_issued(item_on_ground: ItemOnGround)

signal debug_visuals_set(value: bool)

signal task_finished(task: Task)

signal map_ready(map: MainMap)
signal solid_cell_placed(coordinate: Vector2i)
signal solid_cell_removed(coordinate: Vector2i)

signal terrain_placed(coordinate: Vector2i, target_layer: MainMap.Layers,
						terrain_set_id: int, terrain_id: int, is_solid: bool)

signal construction_selected(item: Item)
signal dismantle_selected()

signal item_placed_on_ground(item: ItemOnGround, item_position: Vector2)
signal item_removed_from_ground(item: ItemOnGround, item_position: Vector2)
