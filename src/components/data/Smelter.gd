class_name SmelterComponent extends Component

@export var smelting_time_modifier: float = 1.0
@export var supported_recipes: Array[Recipe]

var _inventory: InventoryComponent = InventoryComponent.new()
var jobs: Array[SmeltingJob]

func _init() -> void:
	id = Components.Id.Smelter

func on_enter() -> void:
	_inventory.set_owner(component_owner)

func get_inventory() -> InventoryComponent:
	return _inventory

func add_job(smelting_job: SmeltingJob) -> void:
	jobs.append(smelting_job)
	smelting_job.start(self)

func smelt(amount: float) -> void:
	pass
