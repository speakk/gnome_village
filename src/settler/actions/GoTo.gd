class_name GoToActorAction extends ActorAction

var target_coordinate: Vector2i

func initialize(params: Variant) -> ActorAction:
	target_coordinate = params.target_coordinate
	return self

func process_action(actor: Settler, delta: float) -> void:
	if actor.global_position.distance_to(Globals.get_map().coordinate_to_global_position(target_coordinate)) < actor.AT_DISTANCE:
		finished.emit(self)
	
	var target_position := Globals.get_map().coordinate_to_global_position(target_coordinate)
	var direction := actor.global_position.direction_to(target_position)
	actor.velocity = direction * actor.walk_speed
	
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target.distance_squared_to(actor.global_position) > 0.01:
		actor.look_at(look_at_target, Vector3.UP, true)
