extends Resource

class_name Item

enum RenderingType {
	Terrain, None, Model
}

enum SpecialFeatures {
	Door, FarmPlot
}

@export_group("Visuals")
@export var rendering_type: RenderingType = RenderingType.Model

@export_subgroup("3D")
@export var model: PackedScene

@export_subgroup("Terrain")
@export var target_layer: MainMap3D.Layers = MainMap3D.Layers.Blueprint
@export var mesh_id: MapMeshes.Id

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
@export var provides: Array[ItemRequirement]
@export var components: Array[Component] = [preload("res://src/components/data/Selectable.tres")]
