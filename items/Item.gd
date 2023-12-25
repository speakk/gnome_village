extends Resource

class_name Item

enum RenderingType {
	Tile, Terrain, Sprite
}

@export_group("Visuals")
@export var rendering_type: RenderingType = RenderingType.Sprite

@export_subgroup("Sprite")
@export var texture: Texture2D = preload("res://assets/materials.png")
@export var hframes: int = 2
@export var vframes: int = 1
@export var frame: int = 0

@export_subgroup("Tile")
@export var tileset_id: int
@export var tile_id: int

@export_subgroup("Terrain")
@export var terrain_set_id: int
@export var terrain_id: int
@export var target_layer: MainMap.Layers

@export_category("Behaviour")
@export var can_be_picked: bool = true
@export var can_be_constructed: bool = false
@export var crafting_requirements: Array[ItemRequirement] = []
