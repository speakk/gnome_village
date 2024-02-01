class_name Zone extends Node2D

var zone_name: String
var zone_type: ZoneManager.ZoneType
var tick_rate: float = 1.0 # Seconds

var tick_timer: float = tick_rate

# Use dictionary because Arrays don't have any "unique" or set related functionality
var coordinates: Dictionary

func _ready() -> void:
	# Ensure zone is in zone group because group inheritance doesn't work
	add_to_group("zone")
	
func _process(delta: float) -> void:
	tick_timer -= delta
	if tick_timer <= 0:
		tick_zone()
		tick_timer = tick_rate

func set_zone_name(_zone_name: String) -> void:
	zone_name = _zone_name

func clean_up() -> void:
	push_warning("Zone clean up not implemented for a zone")

func add_coordinates(new_tiles: Array[Vector2i]) -> void:
	for tile in new_tiles:
		coordinates[tile] = true
		
	Events.zone_updated.emit(self)

func get_coordinates() -> Array[Vector2i]:
	var coordinate_vecs: Array[Vector2i]
	coordinate_vecs.assign(coordinates.keys())
	return coordinate_vecs

func tick_zone() -> void:
	push_warning("tick_zone not implemented for a zone")
