class_name Entity extends Resource

signal delete_called

var component_container: ComponentContainer = ComponentContainer.new()
var definition: EntityDefinition

# Ease of access READ ONLY except from world pos component
var global_position: Vector3

var default_components: Array[Component] = [
	SelectableComponent.new(),
	DisplayNameComponent.new(),
	WorldPositionComponent.new(),
	ItemAmountComponent.new(),
	ShapeComponent.new()
]

static func from_definition(entity_definition: EntityDefinition) -> Entity:
	var entity := Entity.new()
	entity.definition = entity_definition
	
	return entity

func on_enter() -> void:
	component_container.component_owner = self
	set_item_components()

func set_item_components() -> void:
	if definition:
		for default_component: Component in default_components:
			component_container.add_component(default_component, true)
			
		for component: Component in definition.components:
			component_container.add_component(component, true)

		var display_name_component: DisplayNameComponent = component_container.get_by_id(Components.Id.DisplayName)
		if display_name_component:
			display_name_component.display_name = definition.display_name

		var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
		if item_amount:
			item_amount.item = definition
			if item_amount.amount == 0:
				item_amount.amount = 1

func serialize() -> Dictionary:
	var dict := {}
	if definition:
		dict["definition"] = definition.serialize()
	dict["component_container"] = component_container.serialize()
	dict["save_id"] = SaveSystem.get_save_id(self)
	
	return dict

static func static_deserialize(dict: Dictionary) -> Entity:
	var entity: Entity = Entity.new()
	
	entity._should_set_components = false
	
	entity.component_container.component_owner = entity
	entity.component_container.deserialize(dict["component_container"])
	entity.set_meta("save_id", dict["save_id"])
	entity.deserialize(dict)
	SaveSystem.register_entity_reference(entity)
	return entity

func deserialize(dict: Dictionary) -> void:
	pass

func delete() -> void:
	Events.item_removed_from_ground.emit(self)
	component_container.on_delete()
	component_container = null
	delete_called.emit()
