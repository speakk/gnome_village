extends Sprite2D

@onready var original_rotation := rotation_degrees

const SELF_CLOSE_TIME := 2.0
const SELF_CLOSE_SPEED := 0.5
var self_close_timer := 0.0

var is_locked := false
var open_amount := 0.0:
	set(new_amount):
		rotation_degrees = original_rotation + new_amount * 90
		#$LightOccluder2D.ro
		open_amount = new_amount

func correct_orientation() -> void:
	var self_coordinate := (Globals.get_map() as MainMap).global_position_to_coordinate(global_position)
	var surrounding_coordinates := PathFinder.get_surrounding_coordinates(self_coordinate, false)
	for coordinate in surrounding_coordinates:
		var coordinate_entities := Globals.get_map().get_map_entities(coordinate)
		for coordinate_entity in coordinate_entities:
			if coordinate_entity.item_id == Items.Id.WoodenWall:
				var angle_to := Vector2(self_coordinate).angle_to_point(Vector2(coordinate))
				global_rotation = angle_to + PI/2
				if is_equal_approx(angle_to, 2*PI) or is_equal_approx(angle_to, 0):
					offset.x = 0
					offset.y = -8
					position.x = -8
					position.y = 8
				else:
					offset.x = 0
					offset.y = -8 
					position.x = 0
					position.y = 8
				return

func _ready() -> void:
	correct_orientation()
	Events.map_changed.connect(_map_changed)

# TODO: Some kind of bigger cells/quadtree instead of literally
# doing this for every door on the whole map
func _map_changed(coordinate: Vector2i) -> void:
	if global_position.distance_to(Globals.get_map().global_position_to_coordinate(coordinate)) < MainMap.CELL_SIZE.x:
		correct_orientation()

func open_by_amount(amount: float) -> void:
	self_close_timer = SELF_CLOSE_TIME
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
