class_name EntityScene extends Node3D

var component_container: ComponentContainer

#static var ENTITY_SCENE := preload("res://src/entities/entity/EntityScene.tscn")

var show_amount_number := true

var definition: EntityDefinition:
	set(new_definition):
		definition = new_definition

func _amount_changed(new_amount: int) -> void:
	if show_amount_number:
		$ItemAmountLabel.text = "%s" % new_amount
		if new_amount > 1:
			$ItemAmountLabel.show()
		else:
			$ItemAmountLabel.hide()
	
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	pass
	#var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
	#item_amount.amount_changed.connect(func(new_amount: int) -> void:
		#if new_amount > 1:
			#$ItemAmountLabel.text = new_amount
			#$ItemAmountLabel.show()
		#else:
			#$ItemAmountLabel.hide()
		#
		#if new_amount <= 0:
			#queue_free()
		#)
