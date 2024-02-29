class_name BuildAction extends ActionLeaf

var target: ConstructableComponent
var _action: BuildActorAction

var _done := false

@onready var ACTION := preload("res://src/settler/actions/Build.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		return SUCCESS
	
	if not actor.can_reach_target(target.get_owner().global_position):
		return FAILURE
	
	if not _action:
		_action = ACTION.new().initialize({ constructable_component = target })
		target.finished.connect(func() -> void: _done = true)
		actor.add_action(_action)
		
	return RUNNING
