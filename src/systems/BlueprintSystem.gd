class_name BlueprintSystem extends System

func _ready() -> void:
	super._ready()
	Events.construction_started.connect(_construction_started)

func _component_added(container: ComponentContainer, new_component: Component) -> void:
	var has_blueprint := container.has_component(Components.Id.Blueprint)
	if has_blueprint:
		for component: Component in container.get_all():
			if component.has_method("set_blueprint"):
				component.set_blueprint(true)

func _component_removed(container: ComponentContainer, new_component: Component) -> void:
	var has_blueprint := container.has_component(Components.Id.Blueprint)
	if not has_blueprint:
		for component: Component in container.get_all():
			if component.has_method("set_blueprint"):
				component.set_blueprint(false)

func _construction_started(container: ComponentContainer) -> void:
	container.remove_component(Components.Id.Blueprint)
