class_name PlantComponent extends Component

var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

## How long does it take to progress to next growth stage (in seconds)
@export var growth_stage_time: float = 2.0
@export var growth_stage_time_variance: float = 0.5
@export var growth_stages: Array[GrowthStage]
@export var growth_requirements: Array[ItemRequirement]


var managed_by_player := false

var _actual_grow_time: float


var current_growth_timer: float = 0.0
var current_growth_stage_index: int = -1:
	set(new_value):
		current_growth_stage_index = new_value
		advanced_growth_stage.emit(current_growth_stage_index)

var current_growth_scene: Variant

signal satisfies_growth_requirements
signal lacks_growth_requirements
signal advanced_growth_stage(new_stage_index: int)
signal matured

var grows_in: GrowthSpotComponent

func _init() -> void:
	id = Components.Id.Plant
	subscriptions.append(Subscription.new(id, Components.Id.Spread, _set_spread_component))

func recalculate_actual_growth_time() -> void:
	_actual_grow_time = growth_stage_time + randf_range(-growth_stage_time_variance/2, growth_stage_time_variance/2)

func on_enter() -> void:
	recalculate_actual_growth_time()

func _set_spread_component(spread_component: SpreadComponent) -> void:
	spread_component.spreads.connect(_spreads)
	matured.connect(func() -> void: spread_component.set_active(true))
	if is_mature():
		spread_component.set_active(true)
	else:
		spread_component.set_active(false)

static func create_growth_spot(new_position: Vector3) -> ItemOnGround:
	var new_grows_in_entity: ItemOnGround = load("res://src/items/item_on_ground/ItemOnGround.tscn").instantiate()
	Events.request_entity_add.emit(new_grows_in_entity)
	var grows_container: ComponentContainer = new_grows_in_entity.component_container
	grows_container.add_component(GrowthSpotComponent.new())
	var inventory: InventoryComponent = grows_container.add_component(InventoryComponent.new())
	inventory.add_item_amount(Items.Id.Water, 300)
	inventory.items_can_be_picked = false
	var grows_world_pos_component: WorldPositionComponent = grows_container.add_component(WorldPositionComponent.new())
	grows_world_pos_component.current_position = new_position
	return new_grows_in_entity

func _spreads(coordinate: Vector2i) -> void:
	var global_pos: Vector3 = Globals.get_map().coordinate_to_global_position(coordinate)
	var new_grows_in_entity := PlantComponent.create_growth_spot(global_pos)
	
	var new_plant: ItemOnGround = ITEM_ON_GROUND.instantiate()
	Events.request_entity_add.emit(new_plant)
	#new_plant.initialize(get_owner().item_id)
	new_plant.item = get_owner().item.duplicate(true)
	var comp_container: ComponentContainer = new_plant.component_container
	comp_container.get_by_id(Components.Id.Plant).grows_in = new_grows_in_entity.component_container.get_by_id(Components.Id.GrowthSpot)
	WorldPositionComponent.set_world_position(new_plant, global_pos)
	

func is_mature() -> bool:
	#print("Current stage: %s vs stages size: %s" % [current_growth_stage_index, growth_stages.size()])
	return current_growth_stage_index >= growth_stages.size()

func has_growth_requirements() -> bool:
	if not grows_in:
		return false
	
	for growth_requirement in growth_requirements:
		var satisfies_requirement := false
		for growth_provided: ItemAmountComponent in grows_in.growth_requirement_inventory.get_items():
			if growth_provided.item_id == growth_requirement.item_id \
			and growth_provided.amount >= growth_requirement.amount:
				satisfies_requirement = true
				break
		
		if not satisfies_requirement:
			return false
	
	return true

func consume_growth_requirements() -> void:
	for growth_requirement in growth_requirements:
		grows_in.consume_growth_requirement(growth_requirement.item_id, growth_requirement.amount)

func advance_growth_stage() -> void:
	if not is_mature():
		current_growth_stage_index += 1
		
		if is_mature():
			matured.emit()
			if managed_by_player:
				Events.plant.matured.emit(self)

var lacks_growth_requirements_emitted := false

func process_component(delta: float) -> void:
	if not is_mature():
		if has_growth_requirements():
			lacks_growth_requirements_emitted = false
			satisfies_growth_requirements.emit()
			current_growth_timer += delta
			if current_growth_timer > _actual_grow_time:
				advance_growth_stage()
				consume_growth_requirements()
				current_growth_timer = 0
				recalculate_actual_growth_time()
		elif not lacks_growth_requirements_emitted:
			#print("Emitting lacks")
			lacks_growth_requirements_emitted = true
			lacks_growth_requirements.emit()
			
			if managed_by_player:
				Events.plant.lacks_growth_requirement.emit(grows_in)
