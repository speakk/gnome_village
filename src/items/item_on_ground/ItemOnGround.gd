extends Node3D

class_name ItemOnGround

@onready var ITEM_ON_GROUND := load("res://src/items/item_on_ground/ItemOnGround.tscn")

@onready var component_container: ComponentContainer = $ComponentContainer

var item_scene: Node3D
var item: Item

var item_id: Items.Id:
	set(new_item_id):
		item_id = new_item_id
		item = Items.get_by_id(item_id)

var _initial_state: Variant

func save() -> Dictionary:
	var save_dict := {
		"position_x" = global_position.x,
		"position_y" = global_position.y,
		"item_id" = item_id,
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	global_position.x = save_dict["position_x"]
	global_position.y = save_dict["position_y"]
	item_id = save_dict["item_id"]

	Events.item_placed_on_ground.emit(self, global_position)

func initialize(_item_id: Items.Id, _amount: int = 1) -> ItemOnGround:
	item_id = _item_id
	
	set_item_components()
	component_container.get_by_id(Components.Id.ItemAmount).amount = _amount

	return self
	
func _amount_changed(new_amount: int) -> void:
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

func generate_drops() -> void:
	for item_drop in item.item_drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_item_on_ground := ITEM_ON_GROUND.instantiate() as ItemOnGround
			# TODO: Randomize position slightly
			get_parent().add_child(new_item_on_ground)
			new_item_on_ground.initialize(item_drop.item_id, amount)
			var position_component: WorldPositionComponent = new_item_on_ground.component_container.get_by_id(Components.Id.WorldPosition)
			position_component.current_position = global_position
			WorldPositionComponent.set_world_position(new_item_on_ground, global_position)
			
func place_at_coordinate(coordinate: Vector2i) -> void:
	var new_position := Globals.get_map().coordinate_to_global_position(coordinate)
	WorldPositionComponent.set_world_position(self, new_position)

func set_item_components() -> void:
	for component in item.components:
		component_container.add_component(component)

	component_container.get_by_id(Components.Id.DisplayName).display_name = item.display_name
