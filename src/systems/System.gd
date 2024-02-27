class_name System extends Node

func _ready() -> void:
	Events.component.added.connect(_component_added)
	Events.component.removed.connect(_component_removed)

func _component_added(container: ComponentContainer, component: Component) -> void:
	push_warning("Abstract System _component_added called - Did you forget to implement it for the specific system?")

func _component_removed(container: ComponentContainer, component: Component) -> void:
	pass
