extends ActionLeaf

var _action: GoToActorAction
var _done := false

@onready var ACTION := preload("res://settler/actions/GoTo.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	if _done:
		_done = false
		_action = null
		return SUCCESS
	
	if not _action:
		var current_path_index := blackboard.get_value("current_path_index") as int
		print("Making new goto action with current_path_index", current_path_index)
		var path := blackboard.get_value("path") as PackedVector2Array
		print("Path size: ", path.size())
		if current_path_index <= path.size() - 1:
			print("Path index was undex size, proceeding")
			var current_target_coordinate := path[current_path_index]
			
			_action = ACTION.new().initialize({ target_coordinate =  current_target_coordinate })
			_action.finished.connect(func(__action: ActorAction) -> void:
				blackboard.set_value("current_path_index", current_path_index + 1)
				blackboard.erase_value("blocking_door")
				_done = true
			)
			blackboard.set_value("goto_action", _action)
			actor.add_action(_action)
		else:
			return SUCCESS
		
	return RUNNING
