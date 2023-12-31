extends Resource

class_name Item

enum RenderingType {
	Tile, Terrain, Sprite, None
}

enum SpecialFeatures {
	Door
}

@export_group("Visuals")
@export var rendering_type: RenderingType = RenderingType.Sprite

@export_subgroup("Sprite")
@export var texture: Texture2D = load("res://assets/materials.png")
@export var hframes: int = 2
@export var vframes: int = 1
@export var frame: int = 0
@export var origin: Vector2 = Vector2(0.0, 0.0)
@export_category("Shadow")
@export var cast_shadow_enabled: bool = false
@export var cast_shadow_origin: Vector2 = Vector2(0, 0)

@export_subgroup("Tile")
@export var tileset_id: int = 0
@export var tile_id: int = 0

@export_subgroup("Terrain")
@export var terrain_set_id: int = 0
@export var terrain_id: int = 0
@export var target_layer: MainMap.Layers = MainMap.Layers.Blueprint

@export_category("Behaviour")
@export var display_name: String = ""
@export var scene: PackedScene = null
@export var can_be_picked: bool = true
@export var can_be_constructed: bool = false
@export var can_be_dismantled: bool = false
@export var is_solid: bool = false
@export var durability: int = 10
@export var crafting_requirements: Array[ItemRequirement] = []
@export var item_drops: Array[ItemDrop] = []
@export var special_features: Array[SpecialFeatures]
