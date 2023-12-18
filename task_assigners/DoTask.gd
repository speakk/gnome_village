extends ActionLeaf

@warning_ignore("untyped_declaration")
func tick(actor: Object, blackboard: Blackboard):
	var settler = get_parent().settler as Settler
	if not settler:
		return FAILURE
	
	var task = get_parent().task
	
	if not task:
		return FAILURE
		
	if task.is_finished:
		return SUCCESS
	
	if settler.get_current_task() != task and !task.is_finished:
		add_child(task)
		task.actor_node_path = settler.get_path()
		settler.start_task(task)
	
	#return settler.tick_current_task()
	return settler.get_task_status()
