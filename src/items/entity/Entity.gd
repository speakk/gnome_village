extends Node3D

class_name Entity

@onready var ENTITY := load("res://src/items/entity/Entity.tscn")

@onready var component_container: ComponentContainer = $ComponentContainer

var show_amount_number := true

var item_scene: Node3D
var item: Item:
	set(new_item):
		item = new_item
		set_item_components()

var _initial_state: Variant

func save() -> Dictionary:
	var save_dict := {
		"position_x" = global_position.x,
		"position_y" = global_position.y,
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	global_position.x = save_dict["position_x"]
	global_position.y = save_dict["position_y"]

	Events.item_placed_on_ground.emit(self, global_position)

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
	set_notify_transform(true)

func _exit_tree() -> void:
	Events.item_removed_from_ground.emit(self)


func place_at_coordinate(coordinate: Vector2i) -> void:
	var new_position := Globals.get_map().coordinate_to_global_position(coordinate)
	WorldPositionComponent.set_world_position(self, new_position)

func set_item_components() -> void:
	for component: Component in item.components:
		component_container.add_component(component)

	# This untyped display_name_component is here because of an OBSCURE bug
	# If you type display_name_component here, basically all components become invalid
	# and everything breaks
	@warning_ignore("untyped_declaration")
	var display_name_component = component_container.get_by_id(Components.Id.DisplayName)
	display_name_component.display_name = item.display_name

	var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
	item_amount.amount_changed.connect(func(new_amount: int) -> void:
		if new_amount > 1:
			$ItemAmountLabel.text = new_amount
			$ItemAmountLabel.show()
		else:
			$ItemAmountLabel.hide()
		
		if new_amount <= 0:
			queue_free()
		)
	
	item_amount.item = item
	item_amount.amount = 1
