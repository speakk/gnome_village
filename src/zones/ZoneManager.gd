class_name ZoneManager extends Node2D

enum ZoneType {
	Farming
}

var zone_scenes: Dictionary = {
	ZoneType.Farming: preload("res://src/zones/zone_types/FarmingZone.tscn")
}

static var zone_names: Dictionary = {
	ZoneType.Farming: "Farming"
}

func _ready() -> void:
	Events.zone_add_pressed.connect(_zone_add_pressed)
	Events.zone_delete_pressed.connect(_zone_delete_pressed)

static func get_zone_type_name(zone_type: ZoneType) -> String:
	return zone_names[zone_type]

func _zone_add_pressed(zone_type: ZoneType, zone_name: String) -> void:
	var zone: Zone = zone_scenes[zone_type].instantiate()
	zone.set_zone_name(zone_name)
	$Zones.add_child(zone)
	Events.zone_added.emit(zone)
	Events.zone_list_changed.emit()

func _zone_delete_pressed(zone: Zone) -> void:
	zone.clean_up()
	Events.zone_deleted.emit(zone)
	$Zones.remove_child(zone)
	zone.queue_free()
	await get_tree().physics_frame
	Events.zone_list_changed.emit()
