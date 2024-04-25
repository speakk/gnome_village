class_name SmelterComponent extends Component

@export var smelting_time_modifier: float = 1.0
@export var supported_recipes: Array[Recipe]

var _inventory: InventoryComponent
var jobs: Array[SmeltingJob]

func _init() -> void:
	id = Components.Id.Smelter
	subscriptions.append(
		Subscription.new(self.id, Components.Id.Inventory,
		func(inventory_component: InventoryComponent) -> void:
			_inventory = inventory_component
			))

func smelt(_delta: float) -> void:
	pass
