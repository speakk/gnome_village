class_name ConstructableComponent extends Component

@export var requirements: Array[ItemRequirement]
@export var solid_when_started := false
@export var can_be_dismantled := true

@export var max_durability: float = 10.0

var _current_durability: float = 10.0:
	set(new_value):
		_current_durability = new_value
		if _current_durability <= 0 and not _no_durability_emitted:
			no_durability_left.emit()
			component_owner.queue_free()
			_no_durability_emitted = true

var reserved_for_dismantling := false

var _inventory: InventoryComponent = InventoryComponent.new()

var is_finished := false
var is_started := false

signal progress_changed(new_value: float)
signal finished
signal started

signal no_durability_left
var _no_durability_emitted := false

func _init() -> void:
	id = Components.Id.Constructable
	_current_durability = max_durability
	subscriptions.append(
		Subscription.new(id, Components.Id.Blueprint, func(blueprint: BlueprintComponent) -> void:
			blueprint.removed.connect(func() -> void:
				if not is_started:
					Events.blueprint_cancel_issued.emit(component_owner)
			)
			
			started.connect(func() -> void:
				if get_container().has_component(Components.Id.Blueprint):
					get_container().remove_component(Components.Id.Blueprint)
			)
			)
	)

func set_owner(_owner: Node) -> void:
	super.set_owner(_owner)
	_inventory.set_owner(_owner)

var _current_progress: float:
	set(new_value):
		progress_changed.emit(new_value)
		_current_progress = new_value
		
		if _current_progress > 0:
			if not is_started:
				is_started = true
				Events.construction_started.emit(get_container())
				if solid_when_started:
					get_container().add_component(SolidComponent.new())
				started.emit()
		
		if _current_progress >= 1.0:
			if not is_finished:
				Events.construction_finished.emit(get_owner())
				finished.emit()
				is_finished = true

func has_requirements() -> bool:
	for requirement in requirements:
		var has_requirement := _inventory.has_item_requirement(requirement)
		if not has_requirement:
			return false
	
	return true

func supply_item_amount(item_amount: ItemAmountComponent) -> void:
	_inventory.add_item_amount(item_amount.item, item_amount.amount)

func increase_progress(amount: float) -> void:
	_current_progress += amount

func get_inventory() -> InventoryComponent:
	return _inventory

func reduce_durability(amount: float) -> void:
	_current_durability -= amount
