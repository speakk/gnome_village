extends Zone

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

func _init() -> void:
	zone_type = ZoneManager.ZoneType.Farming

func tick_zone() -> void:
	print("Ticking zone!")
	for coordinate in get_coordinates():
		print("Coordinate: ", coordinate)
		var entities := Globals.get_map().get_map_entities(coordinate)
		if entities.size() == 0:
			print("No entities thusly:")
			var farm_plot := (ITEM_ON_GROUND.instantiate() as ItemOnGround)
			get_tree().root.get_node("Main").get_node("Entities").add_child(farm_plot)
			farm_plot.global_position = Globals.get_map().coordinate_to_global_position(coordinate)
			farm_plot.initialize(Items.Id.FarmPlot, 1, ItemOnGround.ItemState.Blueprint)
			Events.blueprint_placed.emit(coordinate, farm_plot)
