class_name PlantComponent extends Component

signal satisfies_growth_requirements
signal lacks_growth_requirements
signal advanced_growth_stage(new_stage_index: int)
signal matured

## How long does it take to progress to next growth stage (in seconds)
@export var growth_stage_time: float = 2.0
@export var growth_stage_time_variance: float = 0.5
@export var growth_stages: Array[GrowthStage]
@export var growth_requirements: Array[ItemRequirement]

var managed_by_player := false
var lacks_growth_requirements_emitted := false

var _actual_grow_time: float

var current_growth_timer: float = 0.0
var current_growth_stage_index: int = -1:
	set(new_value):
		current_growth_stage_index = new_value
		advanced_growth_stage.emit(current_growth_stage_index)

var current_growth_scene: Variant
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

static func create_growth_spot(new_position: Vector3) -> Entity:
	var new_grows_in_entity: Entity = load("res://src/entities/entity/Entity.tscn").instantiate()
	#var new_grows_in_entity
	Events.request_entity_add.emit(new_grows_in_entity)
	var grows_container: ComponentContainer = new_grows_in_entity.component_container
	grows_container.add_component(GrowthSpotComponent.new())
	var inventory: InventoryComponent = grows_container.add_component(InventoryComponent.new())
	inventory.add_item_amount(preload("res://src/entities/definitions/water.tres"), 300)
	inventory.items_can_be_picked = false
	var grows_world_pos_component: WorldPositionComponent = grows_container.add_component(WorldPositionComponent.new())
	grows_world_pos_component.current_position = new_position
	# TODO: This will not be needed when reworking itemAmount
	# Right now removing this default as this entity doesn't have a definition
	# and thus item amount causes issues in serialization
	return new_grows_in_entity

func _spreads(coordinate: Vector2i) -> void:
	var global_pos: Vector3 = Globals.get_map().coordinate_to_global_position(coordinate)
	var new_grows_in_entity := PlantComponent.create_growth_spot(global_pos)
	
	var new_plant: Entity = Entity.from_definition(get_owner().definition)
	Events.request_entity_add.emit(new_plant)
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
			if growth_provided.item == growth_requirement.item \
			and growth_provided.amount >= growth_requirement.amount:
				satisfies_requirement = true
				break
		
		if not satisfies_requirement:
			return false
	
	return true

func consume_growth_requirements() -> void:
	for growth_requirement in growth_requirements:
		grows_in.consume_growth_requirement(growth_requirement.item, growth_requirement.amount)

func advance_growth_stage() -> void:
	if not is_mature():
		current_growth_stage_index += 1
		
		if is_mature():
			matured.emit()
			if managed_by_player:
				Events.plant.matured.emit(self)


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



#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["growth_stage_time"] = growth_stage_time
	dict["growth_stage_time_variance"] = growth_stage_time_variance
	dict["growth_stages"] = growth_stages.map(func(growth_stage: GrowthStage) -> Dictionary:
		return growth_stage.serialize()
		)
	dict["growth_requirements"] = growth_requirements.map(func(item_requirement: ItemRequirement) -> Dictionary:
		return item_requirement.serialize()
		)
	
	dict["managed_by_player"] = managed_by_player
	dict["lacks_growth_requirements_emitted"] = lacks_growth_requirements_emitted
	dict["_actual_grow_time"] = _actual_grow_time
	dict["current_growth_timer"] = current_growth_timer
	dict["current_growth_stage_index"] = current_growth_stage_index
		
	if current_growth_scene is PackedScene:
		dict["current_growth_scene_path"] = current_growth_scene.resource_path
	
	if grows_in is GrowthSpotComponent:
		dict["grows_in_owner_id"] = SaveSystem.get_save_id(grows_in.get_owner())
	
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	growth_stage_time = dict["growth_stage_time"] 
	growth_stage_time_variance = dict["growth_stage_time_variance"] 
	growth_stages.assign(dict["growth_stages"].map(func(growth_stage_dict: Dictionary) -> GrowthStage:
		var growth_stage := GrowthStage.new()
		growth_stage.deserialize(growth_stage_dict)
		return growth_stage
		))

	growth_requirements.assign(dict["growth_requirements"].map(func(growth_requirement_dict: Dictionary) -> ItemRequirement:
		var growth_requirement := ItemRequirement.new()
		growth_requirement.deserialize(growth_requirement_dict)
		return growth_requirement
		))
	
	managed_by_player = dict["managed_by_player"] 
	lacks_growth_requirements_emitted = dict["lacks_growth_requirements_emitted"] 
	_actual_grow_time = dict["_actual_grow_time"] 
	current_growth_timer = dict["current_growth_timer"] 
	current_growth_stage_index = dict["current_growth_stage_index"] 
		
	if dict.has("current_growth_scene_path"):
		current_growth_scene = load(dict["current_growth_scene_path"]).instantiate()
	
	if dict.has("grows_in_owner_id"):
		SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(
			dict["grows_in_owner_id"], func(grows_in_owner: Entity) -> void:
				grows_in = grows_in_owner.component_container.get_by_id(Components.Id.GrowthSpot)
		))
	
#endregion
