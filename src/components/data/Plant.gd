class_name PlantComponent extends Component

signal matured

## How long does it take to progress to next growth stage (in seconds)
@export var growth_stage_time: float = 2.0
@export var growth_stages: Array[GrowthStage]

@export var growth_requirements: Array[ItemRequirement]

var current_growth_timer: float = 0.0
var current_growth_stage_index: int = -1

var current_growth_scene: Variant

signal satisfies_growth_requirements
signal lacks_growth_requirements
signal advanced_growth_stage(new_stage_index: int)

var grows_in: GrowthSpotComponent

func _init() -> void:
	id = Components.Id.Plant

func is_mature() -> bool:
	return current_growth_stage_index >= growth_stages.size() - 1

func has_growth_requirements() -> bool:
	if not grows_in:
		return false
		
	for growth_requirement in growth_requirements:
		var satisfies_requirement := false
		for growth_provided: ItemAmountComponent in grows_in.growth_requirement_inventory.get_items():
			if growth_provided.id == growth_requirement.item_id \
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
		advanced_growth_stage.emit(current_growth_stage_index)
		
		if is_mature():
			matured.emit()

var lacks_growth_requirements_emitted := false

func process_component(delta: float) -> void:
	#print("Processing plant")
	if not is_mature():
		#print("Not mature")
		if has_growth_requirements():
			#print("Had growth requirements")
			lacks_growth_requirements_emitted = false
			satisfies_growth_requirements.emit()
			current_growth_timer += delta
			if current_growth_timer > growth_stage_time:
				advance_growth_stage()
				consume_growth_requirements()
				current_growth_timer = 0
		elif not lacks_growth_requirements_emitted:
			#print("Emitting lacks")
			lacks_growth_requirements_emitted = true
			lacks_growth_requirements.emit()
