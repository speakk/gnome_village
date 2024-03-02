class_name GrowthSpotComponent extends Component

signal plant_id_set(new_plant_id: Items.Id)

var has_growing: bool

var growth_requirement_inventory: InventoryComponent

func _init() -> void:
	id = Components.Id.GrowthSpot
	growth_requirement_inventory = InventoryComponent.new()

func consume_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.remove_item_amount(growth_requirement_id, amount)

func increase_growth_requirement(growth_requirement_id: Items.Id, amount: int) -> void:
	growth_requirement_inventory.add_item_amount(growth_requirement_id, amount)

func start_growing_plant(item_id: Items.Id) -> void:
	if not has_growing:
		plant_id_set.emit(item_id)
		has_growing = true
