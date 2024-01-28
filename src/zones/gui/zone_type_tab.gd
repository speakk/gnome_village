extends VBoxContainer

@onready var ZONE_LIST_ITEM := preload("res://src/zones/gui/zone_list_item.tscn")

var zone_type: ZoneManager.ZoneType

func _ready() -> void:
	Events.zone_added.connect(func(_new_zone: Zone) -> void: _refresh_zones())
	Events.zone_deleted.connect(func(_new_zone: Zone) -> void: _refresh_zones())
	Events.zone_list_changed.connect(func() -> void: _refresh_zones())
	
	_refresh_zones()

func set_zone_type(_zone_type: ZoneManager.ZoneType) -> void:
	name = ZoneManager.get_zone_type_name(_zone_type)
	zone_type = _zone_type

func _on_add_zone_button_pressed() -> void:
	var new_name: String = %ZoneNameEdit.text if %ZoneNameEdit.text.length() > 0 else name
	Events.zone_add_pressed.emit(zone_type, new_name)

func _refresh_zones() -> void:
	for child in %ZonesList.get_children():
		child.queue_free()
	
	var zone_nodes := get_tree().get_nodes_in_group("zone") as Array[Node]
	var zones: Array[Zone] = []
	zones.assign(zone_nodes)
	
	for zone in zones:
		print("Righto ", zone.zone_type, " vs : ", zone_type)
		if zone.zone_type == zone_type:
			var zone_list_item: ZoneListItem = ZONE_LIST_ITEM.instantiate()
			zone_list_item.set_zone(zone)
			%ZonesList.add_child(zone_list_item)
	
	if zones.size() <= 0:
		var no_zones_label := Label.new()
		no_zones_label.text = "(No zones)"
		%ZonesList.add_child(no_zones_label)
