extends ActionLeaf

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	if get_parent().task.is_being_worked_on:
		return SUCCESS
		
	var settlers = get_tree().get_nodes_in_group("settler")
	
	# TODO: Make it so that this Node gets an optional heuristic on how to pick which settler
	# does this job - for now just pick first settler that is available for work
	for settler in settlers:
		if settler.is_available_for_work():
			get_parent().settler = settler
			get_parent().task.is_being_worked_on = true
			#blackboard.set_value("settler", settler)
			return SUCCESS
	
	return FAILURE
