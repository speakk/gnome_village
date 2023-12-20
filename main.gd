extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var MATERIAL_ON_GROUND := preload("res://crafting_materials/MaterialOnGround.tscn")

func _ready() -> void:
	#pass
	for i in 100:
		var settler := SETTLER.instantiate()
		settler.global_position = Vector2(randf_range(0, 1280), randf_range(0, 640))
		add_child(settler)

	var material_types: Array[CraftingMaterials.CraftingMaterialId] = [CraftingMaterials.CraftingMaterialId.Wood, CraftingMaterials.CraftingMaterialId.Stone]

	for i in 100:
		var material_on_ground := await (MATERIAL_ON_GROUND.instantiate() as MaterialOnGround).initialize(material_types.pick_random())
		material_on_ground.global_position = Vector2(randf_range(0, 1280), randf_range(0, 640))
		add_child(material_on_ground)
