class_name DismantleAction extends ActionLeaf

var target: ItemOnGround
var _action: DismantleActorAction
var _done := false

@onready var ACTION := preload("res://src/settler/actions/Dismantle.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS
	
	if target and not actor.can_reach_target(target.global_position):
		return FAILURE
	
	if not _action:
		_action = DismantleActorAction.new(actor, { target =  target })
		_action.finished.connect(func(__action: ActorAction) ->void:
			_done = true)
		actor.add_action(_action)
		
	return RUNNING
