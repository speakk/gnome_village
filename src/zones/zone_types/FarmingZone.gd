extends Zone

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

func _init() -> void:
	zone_type = ZoneManager.ZoneType.Farming

func tick_zone() -> void:
	for coordinate in get_coordinates():
		var entities := Globals.get_map().get_map_entities(coordinate)
		if entities.size() == 0:
			var farm_plot := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			get_tree().root.get_node("Main").get_node("Entities").add_child(farm_plot)
			farm_plot.global_position = Globals.get_map().coordinate_to_global_position(coordinate)
			farm_plot.initialize(Items.Id.FarmPlot, 1, ItemOnGround.ItemState.Blueprint)
			Events.blueprint_placed.emit(coordinate, farm_plot)
		else:
			for entity in entities:
				if entity.current_state == ItemOnGround.ItemState.Normal \
				and entity.is_finished():
					entity.item_scene.start_growing_plant(Plants.Id.Potato)
