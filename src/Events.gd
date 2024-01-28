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
signal item_state_changed(item_on_ground: ItemOnGround, previous_state: ItemOnGround.ItemState, new_state: ItemOnGround.ItemState)

signal load_game_called(save_dict: Dictionary)
signal save_game_called(save_dict: Dictionary)

signal current_time_changed(new_time: float)


signal zone_add_pressed(zone_type: ZoneManager.ZoneType, zone_name: String)
signal zone_delete_pressed(zone: Zone)
signal zone_added(zone: Zone)
signal zone_deleted(zone: Zone)
signal zone_list_changed
