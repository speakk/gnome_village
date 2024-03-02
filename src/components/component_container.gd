class_name ComponentContainer extends Node3D

@onready var component_owner := get_parent()
@export var default_components: Array[Component]

var _components: Array[Component]

var _subscriptions: Array[Subscription]

func _ready() -> void:
	for component in default_components:
		add_component(component)

func _exit_tree() -> void:
	for component in _components:
		Events.component.removed.emit(self, component)
		if component.has_method("on_exit"):
			component.on_exit()

func has_component(component_id: Components.Id) -> bool:
	return _components.filter(func(component: Component) -> bool:
		return component.id == component_id
	).size() > 0
		
func get_by_id(component_id: Components.Id) -> Component:
	return _components.filter(func(component: Component) -> bool:
		return component.id == component_id
	)[0]

func get_all() -> Array[Component]:
	var all: Array[Component]
	all.assign(_components)
	return all

func add_component(component: Component) -> void:
	var duplicated: Component = component.duplicate()
	duplicated.set_owner(component_owner)
	_components.append(duplicated)
	Events.component.added.emit(self, duplicated)
	
	if duplicated.has_method("on_enter"):
		duplicated.on_enter()
	
	if duplicated.id == Components.Id.PlantScene:
		print("Adding plantscenecompoennt with subs: %s" % duplicated.subscriptions.size())
	
	for subscription: Subscription in duplicated.get_subscriptions():
		if has_component(subscription.target_id):
			var matching := get_by_id(subscription.target_id)
			subscription.callable.call(matching)
	
	for existing_component in _components:
		for subscription: Subscription in existing_component.get_subscriptions():
			if subscription.target_id == duplicated.id:
				subscription.callable.call(duplicated)

func remove_component(component_id: Components.Id) -> void:
	var matching_all: Array[Component] = _components.filter(func(component: Component) -> bool:
		return component.id == component_id
	)
	
	for matching in matching_all:
		_components.erase(matching)
		Events.component.removed.emit(self, matching)

func subscribe(subscriber_id: Components.Id, target_id: Components.Id, callable: Callable) -> void:
	_subscriptions.append(Subscription.new(subscriber_id, target_id, callable))

func clear() -> void:
	_components.clear()

func _process(delta: float) -> void:
	for component in get_all():
		if component.has_method("process_component"):
			component.process_component(delta)
