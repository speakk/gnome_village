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
				var container: ComponentContainer = entity.component_container
				if container.has_component(Components.Id.GrowthSpot):
					var constructable: ConstructableComponent = container.get_by_id(Components.Id.Constructable)
					if constructable.is_finished:
						var growth_spot: GrowthSpotComponent = container.get_by_id(Components.Id.GrowthSpot)
						growth_spot.start_growing_plant(Items.Id.PotatoPlant)
						# TODO: Absolutely not like this - farm plot component should have this
						#entity.component_container.get_by_id(Components.Id.Scene)._instantiated_scene.start_growing_plant(Items.Id.PotatoPlant)
