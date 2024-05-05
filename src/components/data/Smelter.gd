class_name SmelterComponent extends Component

@export var smelting_time_modifier: float = 1.0
@export var supported_recipes: Array[Recipe]

var _inventory: InventoryComponent = InventoryComponent.new()
var jobs: Array[SmeltingJob]

var _smelting_progress: float = 0
signal smelting_finished

signal job_added(job: SmeltingJob)

func _init() -> void:
	id = Components.Id.Smelter

func on_enter() -> void:
	_inventory.set_owner(component_owner)

func get_inventory() -> InventoryComponent:
	return _inventory

func add_job(smelting_job: SmeltingJob) -> void:
	jobs.append(smelting_job)
	smelting_job.start(self)
	job_added.emit(smelting_job)

func generate_recipe_drop(recipe: Recipe) -> void:
	for item in recipe.produces:
		var entity := Entity.from_definition(item.item)
		Events.request_entity_add.emit(entity)
		var item_amount: ItemAmountComponent = entity.component_container.get_by_id(Components.Id.ItemAmount)
		item_amount.amount = item.amount
		var free_coord: Vector2i = PathFinder.get_closest_free_point(self.coordinate)
		WorldPositionComponent.set_coordinate(entity, free_coord)

func smelt(amount: float) -> void:
	for job in jobs:
		if not job.is_finished:
			_smelting_progress += amount
			if _smelting_progress >= 1:
				generate_recipe_drop(job.recipe)
				smelting_finished.emit()
				_smelting_progress = 0
		
			break
