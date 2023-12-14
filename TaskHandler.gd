extends Node

@onready var BLUEPRINT: PackedScene = preload("res://task_assigners/blueprint.tscn")

func _ready() -> void:
	Events.BlueprintPlaced.connect(_blueprint_placed)

func _process(delta: float) -> void:
	for child in get_children():
		child = child as BeehaveTree
		child.tick()

func _blueprint_placed(target_tile: Vector2i, building_type: BuildingTypes.BuildingType) -> void:
	var blueprint_assigner: BeehaveNode = BLUEPRINT.instantiate()
	%TaskSelector.add_child(blueprint_assigner)
