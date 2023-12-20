extends Node2D

class_name MaterialOnGround

@onready var crafting_materials := preload("res://crafting_materials/CraftingMaterials.gd").new()

@onready var sprite := $Sprite2D as Sprite2D

var material_type: CraftingMaterials.CraftingMaterialId
var crafting_material: CraftingMaterial

func initialize(_material_type: CraftingMaterials.CraftingMaterialId) -> MaterialOnGround:
	material_type = _material_type
	
	return self
	
func _ready() -> void:
	crafting_material = crafting_materials.get_by_id(material_type)
	sprite.texture = crafting_material.texture
	sprite.hframes = crafting_material.hframes
	sprite.vframes = crafting_material.vframes
	sprite.frame = crafting_material.frame
