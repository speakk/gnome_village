extends Node2D

@onready var SETTLER := preload("res://settler/settler.tscn")
@onready var MATERIAL_ON_GROUND := preload("res://crafting_materials/MaterialOnGround.tscn")

func _ready() -> void:
	var test_divider := 1
	var map_size_real_x := MainMap.MAP_SIZE_X * 24 / test_divider
	var map_size_real_y := MainMap.MAP_SIZE_Y * 24 / test_divider
	
	for i in 2:
		var settler := SETTLER.instantiate()
		settler.global_position = Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		add_child(settler)

	#var material_types: Array[CraftingMaterials.CraftingMaterialId] = [CraftingMaterials.CraftingMaterialId.Wood, CraftingMaterials.CraftingMaterialId.Stone]
	var material_types: Array[CraftingMaterials.CraftingMaterialId] = [CraftingMaterials.CraftingMaterialId.Wood]

	for i in 30:
		var material_on_ground := (MATERIAL_ON_GROUND.instantiate() as MaterialOnGround).initialize(material_types.pick_random())
		material_on_ground.global_position = Vector2(randf_range(0, map_size_real_x), randf_range(0, map_size_real_y))
		add_child(material_on_ground)
