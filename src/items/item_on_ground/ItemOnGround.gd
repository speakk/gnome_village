extends Node3D

class_name ItemOnGround

enum ItemState {
	Blueprint, Normal
}

@onready var ITEM_ON_GROUND := preload("res://src/items/item_on_ground/ItemOnGround.tscn")

@onready var component_container: ComponentContainer = $ComponentContainer
@onready var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
@onready var inventory: InventoryComponent = component_container.get_by_id(Components.Id.Inventory)

var item_scene: Node3D
var item: Item

var _dirty: bool

var item_id: Items.Id:
	set(new_item_id):
		item_id = new_item_id
		item = Items.get_by_id(item_id)
		current_durability = item.durability
		max_durability = item.durability
		
		_dirty = true

var current_state: ItemState:
	set(new_state):
		Events.item_state_changed.emit(self, current_state, new_state)
		current_state = new_state
		_dirty = true

func update_rendering() -> void:
	if not item:
		visible = false
		return
		
	visible = true
	
	var coordinate := Globals.get_map().global_position_to_coordinate(global_position)
	
	if item.rendering_type == Item.RenderingType.Model:
		if has_node("model"):
			get_node("model").queue_free()
		var model := item.model.instantiate()
		model.name = "model"
		add_child(model)

	if item.scene:
		if not item_scene:
			var scene := item.scene.instantiate() as Node3D
			scene.name = "scene"
			add_child(scene)
			item_scene = scene

	if current_state == ItemState.Normal:
		# TODO: Rendering really shouldn't update the solid_cell thing
		if item.is_solid:
			Events.solid_cell_placed.emit(coordinate)

var max_durability: float = 10:
	set(new_value):
		$DurabilityProgressBar.max_value = new_value
		max_durability = new_value
		
var current_durability: float = 10:
	set(new_value):
		$DurabilityProgressBar.value = new_value
		current_durability = new_value
		if current_durability < max_durability:
			$DurabilityProgressBar.show()
		else:
			$DurabilityProgressBar.hide()


var finish_emitted := false
var build_progress := 0.0:
	set(new_value):
		$ProgressBar.value = build_progress
		if new_value >= 1.0 or new_value <= 0.0:
			$ProgressBar.hide()
		else:
			$ProgressBar.show()
		
		build_progress = new_value

var reserved_for_picking := false
var reserved_for_dismantling := false:
	set(new_value):
		if new_value:
			$DismantleIndicator.show()
		else:
			$DismantleIndicator.hide()
		
		reserved_for_dismantling = new_value

var _initial_state: Variant

func save() -> Dictionary:
	var save_dict := {
		"position_x" = global_position.x,
		"position_y" = global_position.y,
		"reserved_for_dismantling" = reserved_for_dismantling,
		"reserved_for_picking" = reserved_for_picking,
		"current_durability" = current_durability,
		"max_durability" = max_durability,
		"build_progress" = build_progress,
		"current_state" = current_state,
		"item_amount" = item_amount.save(),
		"item_id" = item_id,
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	global_position.x = save_dict["position_x"]
	global_position.y = save_dict["position_y"]
	reserved_for_dismantling = save_dict["reserved_for_dismantling"]
	reserved_for_picking = save_dict["reserved_for_picking"]
	current_durability = save_dict["current_durability"]
	build_progress = save_dict["build_progress"]
	_initial_state = current_state
	item_id = save_dict["item_id"]
	max_durability = save_dict["max_durability"]
	current_state = save_dict["current_state"]
	item_amount.load_save(save_dict["item_amount"])

	Events.item_placed_on_ground.emit(self, global_position)

func initialize(_item_id: Items.Id, _amount: int = 1) -> ItemOnGround:
	item_id = _item_id
	item_amount.amount = _amount
	
	for provides_item: ItemRequirement in item.provides:
		inventory.add_item_amount(provides_item.item_id, provides_item.amount)
	
	set_item_components()

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
	item_amount.amount_changed.connect(_amount_changed)
	update_rendering()

func _notification(what: int) -> void:
	match what:
		NOTIFICATION_TRANSFORM_CHANGED:
			_dirty = true

func _exit_tree() -> void:
	Events.item_removed_from_ground.emit(self)

func reduce_durability(amount: float) -> void:
	current_durability -= amount

func has_durability_left() -> bool:
	return current_durability > 0

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

func _process(delta: float) -> void:
	if _dirty:
		update_rendering()
		_dirty = false

func set_item_components() -> void:
	component_container.get_by_id(Components.Id.DisplayName).display_name = item.display_name
	for component in item.components:
		component_container.add_component(component)
