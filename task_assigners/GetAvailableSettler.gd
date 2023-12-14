extends ActionLeaf

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	var settlers = get_tree().get_nodes_in_group("settler")
	
	# TODO: Make it so that this Node gets an optional heuristic on how to pick which settler
	# does this job - for now just pick first settler that is available for work
	for settler in settlers:
		get_parent().settler = settler
		#blackboard.set_value("settler", settler)
		return SUCCESS
	
	return FAILURE
