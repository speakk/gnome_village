class_name GoToActorAction extends ActorAction

var target_coordinate: Vector2i

func _init(actor: Settler, params: Dictionary) -> void:
	super._init(actor, params)
	target_coordinate = params.target_coordinate

func process_action(delta: float) -> void:
	if actor.global_position.distance_to(Globals.get_map().coordinate_to_global_position(target_coordinate)) < actor.AT_DISTANCE:
		finished.emit(self)
	
	var target_position := Globals.get_map().coordinate_to_global_position(target_coordinate)
	var direction := actor.global_position.direction_to(target_position)
	actor.velocity = direction * actor.walk_speed
	
	var look_at_target := Vector3(target_position.x, actor.global_position.y, target_position.z)
	if look_at_target.distance_squared_to(actor.global_position) > 0.1:
		actor.look_at(look_at_target, Vector3.UP, true)
