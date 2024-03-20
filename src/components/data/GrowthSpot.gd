class_name GrowthSpotComponent extends Component

signal plant_set(new_plant: Item)

var has_growing: bool

var growth_requirement_inventory: InventoryComponent

var plant_component: PlantComponent

func _init() -> void:
	id = Components.Id.GrowthSpot
	subscriptions.append(
		Subscription.new(id, Components.Id.Inventory,
		func(_inventory: InventoryComponent) -> void:
			growth_requirement_inventory = _inventory
			)
	)

func consume_growth_requirement(growth_requirement_item: Item, amount: int) -> void:
	growth_requirement_inventory.remove_item_amount(growth_requirement_item, amount)

func increase_growth_requirement(growth_requirement_item: Item, amount: int) -> void:
	growth_requirement_inventory.add_item_amount(growth_requirement_item, amount)

func start_growing_plant(item: Item) -> void:
	if not has_growing:
		plant_set.emit(item)
		# TODO: Just implement get_by_id for components as well to avoid this? (already in component_container)
		var components := item.components
		for component in components:
			if component.id == Components.Id.Plant:
				plant_component = component
				break
				
		has_growing = true
