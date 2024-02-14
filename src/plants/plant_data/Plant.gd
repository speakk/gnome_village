class_name Plant extends Resource

@export var display_name: String

## How long does it take to progress to next growth stage (in seconds)
@export var growth_stage_length: float = 2.0
@export var growth_stages: Array[GrowthStage]

@export var growth_requirements: Array[GrowthRequirementAmount]
