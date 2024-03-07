class_name ComponentContainer extends Node3D

var component_owner: Node
@export var default_components: Array[Component]

var _components: Array[Component]

var _components_by_id: Dictionary

var _subscriptions: Array[Subscription]

# Used for optimizing. When amount is 0, no need to process
var _processing_component_amount := 0

func _ready() -> void:
	process_mode = PROCESS_MODE_DISABLED
	component_owner = get_parent()
	for component in default_components:
		add_component(component)

func _exit_tree() -> void:
	for component in _components:
		Events.component.removed.emit(self, component)
		if component.has_method("on_exit"):
			component.on_exit()

func has_component(component_id: Components.Id) -> bool:
	return _components_by_id.has(component_id)
		
func get_by_id(component_id: Components.Id) -> Component:
	if _components_by_id.has(component_id):
		return _components_by_id[component_id]
	
	return null

func get_all() -> Array[Component]:
	return _components

func recheck_processing_mode() -> void:
	if _processing_component_amount > 0:
		process_mode = PROCESS_MODE_INHERIT
	else:
		process_mode = PROCESS_MODE_DISABLED

func add_component(component: Component) -> Component:
	var duplicated: Component = component.duplicate()
	duplicated.set_owner(component_owner)
	_components.append(duplicated)
	_components_by_id[duplicated.id] = duplicated
	Events.component.added.emit(self, duplicated)
	
	if duplicated.has_method("on_enter"):
		duplicated.on_enter()
	
	for subscription: Subscription in duplicated.get_subscriptions():
		if has_component(subscription.target_id):
			var matching := get_by_id(subscription.target_id)
			subscription.callable.call(matching)
	
	for existing_component in _components:
		for subscription: Subscription in existing_component.get_subscriptions():
			if subscription.target_id == duplicated.id:
				subscription.callable.call(duplicated)
	
	if duplicated.has_method("process_component"):
		_processing_component_amount += 1
	
	recheck_processing_mode()
	
	return duplicated

func remove_component(component_id: Components.Id) -> void:
	var matching: Component = _components_by_id[component_id]
	
	if matching.has_method("on_exit"):
		matching.on_exit()
	_components.erase(matching)
	_components_by_id.erase(component_id)
	
	if matching.has_method("process_component"):
		_processing_component_amount -= 1
	
	recheck_processing_mode()
	
	Events.component.removed.emit(self, matching)

func subscribe(subscriber_id: Components.Id, target_id: Components.Id, callable: Callable) -> void:
	_subscriptions.append(Subscription.new(subscriber_id, target_id, callable))

func clear() -> void:
	_components.clear()

func _physics_process(delta: float) -> void:
	for component in get_all():
		if component.has_method("process_component"):
			component.process_component(delta)
