class_name GrowthSpotComponent extends Component

signal plant_id_set(new_plant_id: Items.Id)

var has_growing: bool

var growth_requirement_inventory: InventoryComponent = InventoryComponent.new()

var plant_component: PlantComponent

func _init() -> void:
	id = Components.Id.GrowthSpot

func set_owner(_owner: Node) -> void:
	super.set_owner(_owner)
	growth_requirement_inventory.set_owner(_owner)

func consume_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.remove_item_amount(growth_requirement_id, amount)

func increase_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.add_item_amount(growth_requirement_id, amount)

func start_growing_plant(item_id: Items.Id) -> void:
	if not has_growing:
		plant_id_set.emit(item_id)
		# TODO: Just implement get_by_id for components as well to avoid this? (already in component_container)
		var components := Items.get_by_id(item_id).components
		for component in components:
			if component.id == Components.Id.Plant:
				plant_component = component
				break
				
		has_growing = true
