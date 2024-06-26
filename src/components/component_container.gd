class_name ComponentContainer extends Resource

signal component_added(component: Component)
signal component_removed(component_id: Components.Id)

signal is_processable
signal is_not_processable

var component_owner: Entity

var _components: Array[Component]
var _components_by_id: Dictionary

var _has_entered := false
var _on_enter_queue: Array[Component]

var _subscriptions: Array[Subscription]

# Used for optimizing. When amount is 0, no need to process
var _processing_component_amount := 0

func on_delete() -> void:
	for i in range(_components.size() - 1, -1, -1):
		var component: Component = _components[i]
		Events.component.removed.emit(self, component)
		if component.has_method("on_exit"):
			component.on_exit()
			
		_components.erase(component)
			

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
		is_processable.emit()
	else:
		is_not_processable.emit()

func add_component(component: Component, duplicate: bool = false) -> Component:
	if has_component(component.id):
		remove_component(component.id)
	
	var duplicated: Component
	if duplicate:
		duplicated = component.duplicate()
	else:
		duplicated = component
	
	assert(component_owner, "Component owner missing in add_component")
	duplicated.set_owner(component_owner)
	_components.append(duplicated)
	_components_by_id[duplicated.id] = duplicated
	Events.component.added.emit(self, duplicated)
	
	if duplicated.has_method("on_enter"):
		if _has_entered:
			duplicated.on_enter()
		else:
			_on_enter_queue.append(duplicated)
	
	for subscription: Subscription in duplicated.get_subscriptions():
		if has_component(subscription.target_id):
			var matching := get_by_id(subscription.target_id)
			subscription.callable.call_deferred(matching)
	
	for existing_component in _components:
		for subscription: Subscription in existing_component.get_subscriptions():
			if subscription.target_id == duplicated.id:
				subscription.callable.call_deferred(duplicated)
	
	if duplicated.has_method("process_component"):
		_processing_component_amount += 1
	
	recheck_processing_mode()
	
	component_added.emit(duplicated)
	
	return duplicated

func remove_component(component_id: Components.Id) -> void:
	var matching: Component = _components_by_id[component_id]
	
	matching.on_exit()
	_components.erase(matching)
	_components_by_id.erase(component_id)
	
	if matching.has_method("process_component"):
		_processing_component_amount -= 1
	
	recheck_processing_mode()
	
	component_removed.emit(component_id)
	Events.component.removed.emit(self, matching)

func subscribe(subscriber_id: Components.Id, target_id: Components.Id, callable: Callable) -> void:
	_subscriptions.append(Subscription.new(subscriber_id, target_id, callable))

func on_enter() -> void:
	_has_entered = true
	for component in _on_enter_queue:
		component.on_enter()
	
	_on_enter_queue.clear()

func _physics_process(delta: float) -> void:

	for component in get_all():
		if component.has_method("process_component"):
			component.advance_process_timer(delta)

func serialize() -> Dictionary:
	var serialized_components: Array[Dictionary]
	for component in get_all():
		serialized_components.append(component.serialize())
	
	return {
		components = serialized_components
	}

func clear_components() -> void:
	for component: Component in get_all().duplicate():
		remove_component(component.id)

func deserialize(dict: Dictionary) -> void:
	clear_components()
	for component_dict: Dictionary in dict["components"]:
		var component: Component = Component.static_deserialize(component_dict)
		component = add_component(component)
		component.deserialize(component_dict)
