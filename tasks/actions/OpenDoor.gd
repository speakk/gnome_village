extends ActionLeaf

var door: ItemOnGround
var _action: OpenDoorActorAction
var _done := false

@onready var ACTION := preload("res://settler/actions/OpenDoor.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS
	
	if not actor.can_reach_target(door.global_position):
		return FAILURE
	
	if not _action:
		_action = ACTION.new().initialize({ door = door })
		_action.finished.connect(func(__action: ActorAction) -> void: _done = true)
		actor.add_action(_action)
		
	return RUNNING
