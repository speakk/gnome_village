class_name Item extends Resource

@export_category("Behaviour")
@export var display_name: String = ""
@export var item_drops: Array[ItemDrop] = []
@export var provides: Array[ItemRequirement]
@export var components: Array[Component]
