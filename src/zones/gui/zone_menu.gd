extends PanelContainer

var ZONE_TYPE_TAB := preload("res://src/zones/gui/zone_type_tab.tscn")

func _ready() -> void:
	#var zones = get_tree().get_nodes_in_group("zone")
	for child in %ZoneTypesTabs.get_children():
		child.queue_free()
		
	for zone_type in ZoneManager.ZoneType.values() as Array[ZoneManager.ZoneType]:
		var zone_type_tab := ZONE_TYPE_TAB.instantiate()
		zone_type_tab.set_zone_type(zone_type)
		zone_type_tab.name = ZoneManager.get_zone_type_name(zone_type)
		%ZoneTypesTabs.add_child(zone_type_tab)
