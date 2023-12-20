extends Node2D

class_name MaterialOnGround

@onready var crafting_materials := preload("res://crafting_materials/CraftingMaterials.gd").new()

@onready var sprite := $Sprite2D as Sprite2D

var material_type: CraftingMaterials.CraftingMaterialId
var crafting_material: CraftingMaterial

var reserved_for_picking := false

func initialize(_material_type: CraftingMaterials.CraftingMaterialId, amount: int = 1) -> MaterialOnGround:
	material_type = _material_type
	$Item.amount = 1
	$Item.id = _material_type
	$Item.amount_changed.connect(_amount_changed)
	
	return self
	
func _amount_changed(new_amount: int) -> void:
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	crafting_material = crafting_materials.get_by_id(material_type)
	sprite.texture = crafting_material.texture
	sprite.hframes = crafting_material.hframes
	sprite.vframes = crafting_material.vframes
	sprite.frame = crafting_material.frame
