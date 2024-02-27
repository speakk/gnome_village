extends Zone

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

func _init() -> void:
	zone_type = ZoneManager.ZoneType.Farming

func tick_zone() -> void:
	for coordinate in get_coordinates():
		var entities := Globals.get_map().get_map_entities(coordinate, true)
		if entities.size() == 0:
			var farm_plot := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			Events.request_entity_add.emit(farm_plot)
			farm_plot.initialize(Items.Id.FarmPlot, 1)
			WorldPositionComponent.set_world_position(farm_plot, Globals.get_map().coordinate_to_global_position(coordinate))
			farm_plot.component_container.add_component(BlueprintComponent.new())
			Events.blueprint_placed.emit(coordinate, farm_plot)
		else:
			for entity in entities:
				if entity.current_state == ItemOnGround.ItemState.Normal \
				and entity.is_finished():
					entity.item_scene.start_growing_plant(Plants.Id.Potato)
