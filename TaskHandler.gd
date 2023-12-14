extends Node

@onready var BLUEPRINT: PackedScene = preload("res://task_assigners/blueprint.tscn")

func _ready() -> void:
	Events.BlueprintPlaced.connect(_blueprint_placed)

#func _process(delta: float) -> void:
	#for child in get_children():
		#child = child as BeehaveTree
		#child.tick()
		#print("Ticking")

func _blueprint_placed(target_tile: Vector2i, building_type: BuildingTypes.BuildingType) -> void:
	var blueprint_assigner: BluePrintTaskAssigner = (BLUEPRINT.instantiate() as BluePrintTaskAssigner).initialize(target_tile, building_type)
	add_child(blueprint_assigner)
