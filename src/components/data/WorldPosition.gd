class_name WorldPositionComponent extends Component

var current_coordinate: Vector2i

var _emitted_once := false

signal position_changed(_old_position: Vector3, _global_position: Vector3, _old_coordinate: Vector2i, _coordinate: Vector2i)

var current_position: Vector3:
	set(new_value):
		var new_coordinate := Globals.get_map().global_position_to_coordinate(new_value)
		#if not _emitted_once or current_position.distance_to(new_value) > 0.01:
		if Events:
			Events.world_position_changed.emit(component_owner, current_position, new_value)
		position_changed.emit(current_position, new_value, current_coordinate, new_coordinate)
		_emitted_once = true
			
		current_position = new_value
		current_coordinate = new_coordinate
		
		(func() -> void: get_owner().global_position = current_position).call_deferred()
		#get_owner().global_position = current_position

static func set_world_position(node: Entity, world_position: Vector3) -> void:
	var world_position_component: WorldPositionComponent = node.component_container.get_by_id(Components.Id.WorldPosition)
	world_position_component.current_position = world_position

static func set_coordinate(entity: Entity, _coordinate: Vector2i) -> void:
	var world_position_component: WorldPositionComponent = entity.component_container.get_by_id(Components.Id.WorldPosition)
	var position := Globals.get_map().coordinate_to_global_position(_coordinate)
	world_position_component.current_position = position
	

func _init() -> void:
	id = Components.Id.WorldPosition

func on_enter() -> void:
	_emitted_once = false

func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["x"] = current_position.x 
	dict["y"] = current_position.y
	dict["z"] = current_position.z
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	current_position = Vector3(dict["x"], dict["y"], dict["z"])
