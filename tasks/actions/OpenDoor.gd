extends ActionLeaf

var _door: ItemOnGround
var _action: OpenDoorActorAction
var _done := false

@onready var ACTION := preload("res://settler/actions/OpenDoor.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS

	if not _door:
		var current_path_index := blackboard.get_value("current_path_index") as int
		var next_coordinate := (blackboard.get_value("path") as PackedVector2Array)[current_path_index + 1]
		var entities := Globals.get_map().get_map_entities(next_coordinate)
		for entity in entities:
			if entity.item.special_features.has(Item.SpecialFeatures.Door):
				_door = entity
				break
	
	if not actor.can_reach_target(_door.global_position):
		return FAILURE
	
	if not _action:
		_action = ACTION.new().initialize({ door = _door })
		_action.finished.connect(func(__action: ActorAction) -> void: _done = true)
		actor.add_action(_action)
		
	return RUNNING
