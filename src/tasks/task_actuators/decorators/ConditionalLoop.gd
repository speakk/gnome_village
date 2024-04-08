class_name ConditionalLoopDecorator extends Decorator

@export var blackboard_key: String

## An inverter will return `FAILURE` in case it's child returns a `SUCCESS` status
## code or `SUCCESS` in case its child returns a `FAILURE` status code.

func tick(actor: Node, blackboard: Blackboard) -> int:
	if not blackboard.has_value(blackboard_key):
		blackboard.set_value(blackboard_key, "")
	var c := get_child(0)

	if c != running_child:
		c.before_run(actor, blackboard)

	@warning_ignore("untyped_declaration")
	var response = c.tick(actor, blackboard)
	if can_send_message(blackboard):
		BeehaveDebuggerMessages.process_tick(c.get_instance_id(), response)

	if c is ConditionLeaf:
		blackboard.set_value("last_condition", c, str(actor.get_instance_id()))
		blackboard.set_value("last_condition_status", response, str(actor.get_instance_id()))

	if blackboard.get_value(blackboard_key) == "true":
		c.after_run(actor, blackboard)
		blackboard.erase_value(blackboard_key)
		return SUCCESS
	#
	## TODO Hmm not sure about this
	#if c is ActionLeaf:
		#blackboard.set_value("running_action", c, str(actor.get_instance_id()))
	
	if response == FAILURE:
		return FAILURE
	
	return RUNNING

func get_class_name() -> Array[StringName]:
	var classes := super()
	classes.push_back(&"ConditionalLoopDecorator")
	return classes
