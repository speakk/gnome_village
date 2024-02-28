class_name ConstructableComponent extends Component

@export var requirements: Array[ItemRequirement]
@export var solid_when_started := false

var _inventory: InventoryComponent = InventoryComponent.new()

var is_finished := false

signal progress_changed(new_value: float)
signal finished
signal started

var _started_emitted := false
var _finished_emitted := false

func _init() -> void:
	id = Components.Id.Constructable

func set_owner(_owner: Node) -> void:
	super.set_owner(_owner)
	_inventory.set_owner(_owner)

var _current_progress: float:
	set(new_value):
		progress_changed.emit(new_value)
		_current_progress = new_value
		
		if _current_progress > 0:
			if not _started_emitted:
				Events.construction_started.emit(get_container())
				started.emit()
				_started_emitted = true
		
		if _current_progress >= 1.0:
			if not _finished_emitted:
				Events.construction_finished.emit(get_owner())
				finished.emit()
				_finished_emitted = true
				is_finished = true

func has_requirements() -> bool:
	for requirement in requirements:
		var has_requirement := _inventory.has_item_requirement(requirement)
		if not has_requirement:
			return false
	
	return true

func supply_item_amount(item_amount: ItemAmountComponent) -> void:
	_inventory.add_item_amount(item_amount.item_id, item_amount.amount)

func increase_progress(amount: float) -> void:
	_current_progress += amount

func get_inventory() -> InventoryComponent:
	return _inventory
