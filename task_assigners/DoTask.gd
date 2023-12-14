extends ActionLeaf

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	var settler = get_parent().settler as Settler
	if not settler:
		return FAILURE
	
	var task = get_parent().task
	
	if settler.get_current_task() != task:
		settler.start_task(task)
	
	return settler.tick_current_task()
