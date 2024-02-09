class_name PlayAnimationAction extends ActionLeaf

@export var animation_name: String

@onready var ACTION := preload("res://src/settler/actions/GoTo.gd")

@warning_ignore("untyped_declaration")
func tick(actor: Node, blackboard: Blackboard) -> int:
	actor.play_animation(animation_name)
	return SUCCESS
