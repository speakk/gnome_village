extends Node

var component := ComponentEvents.new()
var plant := PlantEvents.new()
var ui := UiEvents.new()
var world_creation := WorldCreationEvents.new()

class ComponentEvents:
	signal added(container: ComponentContainer, component: Component)
	signal removed(container: ComponentContainer, component: Component)

class PlantEvents:
	signal matured(plant: PlantComponent)
	signal lacks_growth_requirement(growth_spot: GrowthSpotComponent)

class UiEvents:
	signal action_cleared

class WorldCreationEvents:
	signal begin
	signal finished
	signal ground_and_ocean
	signal rocks
	signal rivers
	signal grass
	signal entities

signal blueprint_placed(tile_position: Vector2i, blueprint: Entity)
signal construction_finished(item: Entity)
signal construction_started(container: ComponentContainer)
signal blueprint_cancel_issued(blueprint: Entity)

signal dismantle_issued(entity: Entity)
signal dismantle_finished(entity: Entity)
signal dismantle_cancel_issued(entity: Entity)

signal debug_visuals_set(value: bool)

signal task_finished(task: Task)
signal tasks_changed(tasks: Array[Node])

signal map_ready(map: MainMap)
signal solid_cell_placed(coordinate: Vector2i)
signal solid_cell_removed(coordinate: Vector2i)
signal map_changed(coordinate: Vector2i)
signal mouse_clicked_on_map(click_position: Vector3)
signal mouse_hovered_on_map(hover_position: Vector3)

signal terrain_placed(coordinate: Vector2i, mesh_id: MapMeshes.Id, blueprint: bool)
signal terrain_cleared(coordinate: Vector2i, blueprint: bool)

signal ui_action_selected(ui_action: UiAction)

signal item_placed_on_ground(item: Entity, item_position: Vector2)
signal item_removed_from_ground(item: Entity, item_position: Vector2)

signal world_position_changed(entity: Node3D, old_position: Vector3, new_position: Vector3)
signal entity_selected(entity: Node3D)
signal entity_deselected(entity: Node3D)
signal clear_entity_selections()

signal scene_change_requested(new_scene_id: SceneManager.SceneId)

signal request_entity_add(entity: Node)

signal load_started
signal load_game_called(save_dict: Dictionary)
signal save_game_called(save_dict: Dictionary)
signal new_game_requested
signal quick_load_requested

signal current_time_changed(new_time: float)


signal zone_add_pressed(zone_type: ZoneManager.ZoneType, zone_name: String)
signal zone_delete_pressed(zone: Zone)
signal zone_added(zone: Zone)
signal zone_deleted(zone: Zone)
signal zone_list_changed
signal zone_selected(zone: Zone)
signal zone_updated(zone: Zone)
signal zone_menu_hidden()
