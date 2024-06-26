extends Zone

func _init() -> void:
	zone_type = ZoneManager.ZoneType.Farming

func tick_zone() -> void:
	for coordinate in get_coordinates():
		var entities := Globals.get_map().get_map_entities(coordinate)
		if entities.size() == 0:
			var farm_plot := Entity.from_definition(preload("res://src/entities/definitions/farm_plot.tres"))
			Events.request_entity_add.emit(farm_plot)
			WorldPositionComponent.set_world_position(farm_plot, Globals.get_map().coordinate_to_global_position(coordinate))
			farm_plot.component_container.add_component(BlueprintComponent.new())
			Events.blueprint_placed.emit(coordinate, farm_plot)
		else:
			for entity in entities:
				var container: ComponentContainer = entity.component_container
				if container.has_component(Components.Id.GrowthSpot):
					var constructable: ConstructableComponent = container.get_by_id(Components.Id.Constructable)
					if constructable and constructable.is_finished:
						var growth_spot: GrowthSpotComponent = container.get_by_id(Components.Id.GrowthSpot)
						growth_spot.start_growing_plant(load("res://src/entities/definitions/plants/potato_plant.tres"))
