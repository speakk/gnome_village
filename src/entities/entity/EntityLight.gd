class_name EntityLight extends Resource

var component_container: ComponentContainer = ComponentContainer.new()
var definition: EntityDefinition

var default_components: Array[Component] = [
	SelectableComponent.new(),
	DisplayNameComponent.new(),
	WorldPositionComponent.new(),
	ItemAmountComponent.new(),
	ShapeComponent.new()
]

static func from_definition(entity_definition: EntityDefinition) -> EntityLight:
	var entity_light := EntityLight.new()
	entity_light.definition = entity_definition
	
	return entity_light

func on_enter() -> void:
	component_container.component_owner = self
	set_item_components()

func set_item_components() -> void:
	if definition:
		for default_component: Component in default_components:
			component_container.add_component(default_component)
			
		for component: Component in definition.components:
			component_container.add_component(component)

		var display_name_component: DisplayNameComponent = component_container.get_by_id(Components.Id.DisplayName)
		if display_name_component:
			display_name_component.display_name = definition.display_name

		var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
		if item_amount:
			item_amount.item = definition
			if item_amount.amount == 0:
				item_amount.amount = 1
