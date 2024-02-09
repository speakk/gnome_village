extends Node3D

@onready var original_rotation := rotation_degrees

const SELF_CLOSE_DELAY := 2.0
const SELF_CLOSE_SPEED := 1.2
var self_close_timer := 0.0

var is_locked := false
var open_amount := 0.0:
	set(new_amount):
		rotation_degrees = Vector3(0, original_rotation.y + new_amount * 90, 0)
		open_amount = new_amount

func correct_orientation() -> void:
	var self_coordinate := (Globals.get_map() as MainMap3D).global_position_to_coordinate(global_position)
	var surrounding_coordinates := PathFinder.get_surrounding_coordinates(self_coordinate, false)
	for coordinate in surrounding_coordinates:
		var coordinate_entities := Globals.get_map().get_map_entities(coordinate)
		for coordinate_entity in coordinate_entities:
			if coordinate_entity.item_id == Items.Id.WoodenWall:
				var angle_to := Vector2(self_coordinate).angle_to_point(Vector2(coordinate))
				if is_equal_approx(angle_to, 2*PI) or is_equal_approx(angle_to, 0):
					$MeshInstance3D.rotation_degrees = Vector3(0, 0, 0)
					#offset.x = 0
					#offset.y = -8
					$MeshInstance3D.position.x = 0.5
					$MeshInstance3D.position.z = 0
					position.x = -0.5
					position.z = 0
				else:
					$MeshInstance3D.rotation_degrees = Vector3(0, 90, 0)
					#offset.x = 0
					#offset.y = -8 
					position.x = 0
					position.z = -0.5
					$MeshInstance3D.position.z = 0.5
					$MeshInstance3D.position.x = 0
					
				return

func _ready() -> void:
	correct_orientation()
	Events.map_changed.connect(_map_changed)

# TODO: Some kind of bigger cells/quadtree instead of literally
# doing this for every door on the whole map
func _map_changed(coordinate: Vector2i) -> void:
	if global_position.distance_to(Globals.get_map().coordinate_to_global_position(coordinate)) < MainMap3D.CELL_SIZE.x:
		correct_orientation()

func open_by_amount(amount: float) -> void:
	self_close_timer = SELF_CLOSE_DELAY
	open_amount += amount
	if open_amount >= 1:
		open_amount = 1

func is_open() -> bool:
	return open_amount >= 1.0

func _physics_process(delta: float) -> void:
	if open_amount > 0:
		if self_close_timer <= 0:
			open_amount -= SELF_CLOSE_SPEED * delta

	self_close_timer -= delta
