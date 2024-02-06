class_name GoToActorAction extends ActorAction

var target_coordinate: Vector2i

var path: Variant: # PackedVector2Array | Null
	set(new_path):
		path = new_path
		#if new_path:
			#$Line2D.points = Array(new_path).map(func(point: Vector2i) -> Vector2:
				#return Globals.get_map().coordinate_to_global_position(point)
			#)
		#else:
			#$Line2D.points = []
var current_path_index: int = 0

func initialize(params: Variant) -> ActorAction:
	target_coordinate = params.target_coordinate
	#print("GoTo init with target: ", target_coordinate)
	return self

func update_path(actor: Settler) -> void:
	var map_position_from := Globals.get_map().global_position_to_coordinate(actor.global_position)
	var map_position_to := target_coordinate
	path = PathFinder.get_id_path_to_closest_point(map_position_from, map_position_to)
	current_path_index = 0

func process_action(actor: Settler, delta: float) -> void:
	## TODO: Set a certain amount of attemps before we give up - or something akin to that
	#if not path:
		#update_path(actor)
		#
	#advance_path_index(actor)
	#
	#if path:
		## TODO: Maybe handle this inside actor instead
		#actor.velocity = get_direction_to_next_path_point(actor) * actor.walk_speed
	
	if actor.global_position.distance_to(Globals.get_map().coordinate_to_global_position(target_coordinate)) < actor.AT_DISTANCE:
		finished.emit(self)
	actor.velocity = actor.global_position.direction_to(Globals.get_map().coordinate_to_global_position(target_coordinate)) * actor.walk_speed
	#print("Velocity now, ", actor.global_position, " vs ", Globals.get_map().coordinate_to_global_position(target_coordinate))

func get_direction_to_next_path_point(actor: Settler) -> Vector3:
	var point_position := path[current_path_index] as Vector2i
	return actor.global_position.direction_to(Globals.get_map().coordinate_to_global_position(point_position))

func clear_path() -> void:
	path = null
	current_path_index = 0

func advance_path_index(actor: Settler) -> void:
	if path:
		var distance := actor.global_position.distance_to(Globals.get_map().coordinate_to_global_position(path[current_path_index]))
		if distance < actor.AT_DISTANCE or (current_path_index == path.size() - 2 and distance < actor.REACH_DISTANCE):
			current_path_index += 1
			if current_path_index > path.size() - 1:
				finished.emit(self)

func get_next_path_coordinate() -> Vector2i:
	return path[mini(current_path_index + 1, path.size() - 1)]
