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

var reserved_for_dismantling := false:
	set(new_value):
		reserved_for_dismantling = new_value
		if new_value:
			get_container().add_component(DismantleIndicatorComponent.new())
		else:
			get_container().remove_component(Components.Id.DismantleIndicator)

# TODO: This is the pain point
# BringResource is serializing inventory with get_owner
# then deserializing with.... Hmm
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

func serialize() -> Dictionary:
	var dict := super.serialize()
	
	dict["requirements"] = requirements.map(func(requirement: ItemRequirement) -> Dictionary:
		return requirement.serialize())
	dict["solid_when_started"] = solid_when_started
	dict["is_finished"] = is_finished
	dict["is_started"] = is_started
	dict["can_be_dismantled"] = can_be_dismantled
	dict["max_durability"] = max_durability
	dict["_current_durability"] = _current_durability
	dict["_current_progress"] = _current_progress
	dict["_no_durability_emitted"] = _no_durability_emitted
	dict["_inventory"] = _inventory.serialize()
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	requirements.assign(dict["requirements"].map(func(requirement_dict: Dictionary) -> ItemRequirement:
		var new_requirement := ItemRequirement.new()
		new_requirement.deserialize(requirement_dict)
		return new_requirement
		))
	solid_when_started = dict["solid_when_started"]
	can_be_dismantled = dict["can_be_dismantled"]
	is_finished = dict["is_finished"]
	is_started = dict["is_started"]
	max_durability = dict["max_durability"]
	_current_durability = dict["_current_durability"]
	_current_progress = dict["_current_progress"]
	_no_durability_emitted = dict["_no_durability_emitted"]
	var new_inventory := InventoryComponent.new()
	new_inventory.deserialize(dict["_inventory"])
