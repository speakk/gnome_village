class_name MaterialInfo extends Control

var SINGLE_CATEGORY := preload("res://src/gui/material_info/single_category.tscn")

var _update_buffer_time: float = 1.0
var _update_buffer_timer: float = _update_buffer_time

var _latest_items: Array[Entity]

enum CategoryId {
	Unrefined, Refined
}

class Category:
	var id: CategoryId
	var tags: Array[TagComponent.Tag]
	var name: String

	func _init(_id: CategoryId, _name:String, _tags: Array[TagComponent.Tag]) -> void:
		id = _id
		name = _name
		tags = _tags

var categories := {
	CategoryId.Unrefined: Category.new(
		CategoryId.Unrefined, "Unrefined",
	[TagComponent.Tag.Wood, TagComponent.Tag.Ore]
	),
	CategoryId.Refined: Category.new(
		CategoryId.Refined, "Processed",
	[TagComponent.Tag.Ingot]
	)
}

func _ready() -> void:
	Events.entities.item_amounts_changed.connect(_queue_update)

func _process(delta: float) -> void:
	_update_buffer_timer += delta
	if _update_buffer_timer >= _update_buffer_time and _latest_items.size() > 0:
		_update_materials(_latest_items)
		_latest_items.clear()
		_update_buffer_timer = 0

func _queue_update(all_materials: Array[Entity]) -> void:
	_latest_items = all_materials.duplicate()
	_update_buffer_timer = 0

func _update_materials(all_materials: Array[Entity]) -> void:
	
	var by_category := {}
	for material: Entity in all_materials:
		if not material.component_container:
			continue
		var tag_comp: TagComponent = material.component_container.get_by_id(Components.Id.Tag)
		if tag_comp:
			var fitting_categories: Array[CategoryId]
			for tag in tag_comp.tags:
				for category: Category in categories.values():
					var has_tag := category.tags.has(tag)
					if has_tag:
						if not fitting_categories.has(category.id):
							fitting_categories.append(category.id)
			
			for category_id in fitting_categories:
				if not by_category.has(category_id):
					by_category[category_id] = []
				
				by_category[category_id].append(material)
	
	for child in %CategoryList.get_children():
		child.queue_free()
	
	for category_id: CategoryId in by_category.keys():
		var category: Category = categories[category_id]
		var single_category := SINGLE_CATEGORY.instantiate()
		single_category.set_category(category)
		var entities: Array[Entity]
		entities.assign(by_category[category_id])
		single_category.set_items(entities)
		%CategoryList.add_child(single_category)
