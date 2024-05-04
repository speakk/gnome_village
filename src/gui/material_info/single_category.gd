class_name SingleCategoryView extends Control

var SINGLE_MATERIAL := preload("res://src/gui/material_info/single_material.tscn")

class TypeAmount:
	var definition: EntityDefinition
	var amount: int

func set_category(category: MaterialInfo.Category) -> void:
	%CategoryName.text = category.name

func set_items(materials: Array[Entity]) -> void:
	for child in %MaterialList.get_children():
		child.queue_free()
	
	var by_definition := {}
	
	for entity in materials:
		if not by_definition.has(entity.definition):
			var type_amount := TypeAmount.new()
			type_amount.definition = entity.definition
			type_amount.amount = 0
			by_definition[entity.definition] = type_amount
		
		by_definition[entity.definition].amount += entity.component_container.get_by_id(Components.Id.ItemAmount).amount
	
	for type_amount: TypeAmount in by_definition.values():
		var single_material := SINGLE_MATERIAL.instantiate()
		single_material.set_type_amount(type_amount)
		%MaterialList.add_child(single_material)
