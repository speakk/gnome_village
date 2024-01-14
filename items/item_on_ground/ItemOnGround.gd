extends Node2D

class_name ItemOnGround

enum ItemState {
	Blueprint, Normal
}

@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")

@onready var sprite := $Sprite2D as Sprite2D
@onready var occluder := $LightOccluder2D as LightOccluder2D
@onready var itemAmount := $ItemAmount as ItemAmount

var item_scene: Node2D

var item_id: Items.Id
var item: Item
var current_state: ItemState:
	set(new_state):
		var coordinate := Globals.get_map().global_position_to_coordinate(global_position)
		if item:
			if new_state == ItemState.Normal:
				#print("New state is Normal")
				if item.is_solid:
					Events.solid_cell_placed.emit(coordinate)
					
				if item.rendering_type == Item.RenderingType.Terrain:
					#print("Clearing terrain and adding to normal layer")
					Events.terrain_placed.emit(coordinate, item.target_layer, item.terrain_set_id, item.terrain_id, item.is_solid, self)
					Events.terrain_cleared.emit(coordinate, MainMap.Layers.Blueprint, item.terrain_set_id)
				elif item.rendering_type == Item.RenderingType.Sprite:
					$Sprite2D.modulate = Color.WHITE
				elif item.rendering_type == Item.RenderingType.None and item.scene:
					get_node("scene").modulate = Color.WHITE
			
			if new_state == ItemState.Blueprint:
				#print("New state is blueprint!")
				if item.rendering_type == Item.RenderingType.Terrain:
					#print("Setting blueprint terrain")
					Events.terrain_placed.emit(coordinate, MainMap.Layers.Blueprint, item.terrain_set_id, item.terrain_id, item.is_solid, self)
					Events.terrain_cleared.emit(coordinate, item.target_layer, item.terrain_set_id)
				elif item.rendering_type == Item.RenderingType.Sprite:
					$Sprite2D.modulate = Color(0.6, 0.6, 1.0, 0.5)
				elif item.rendering_type == Item.RenderingType.None and item.scene:
					get_node("scene").modulate = Color(0.6, 0.6, 1.0, 0.5)
			
		Events.item_state_changed.emit(self, current_state, new_state)
		current_state = new_state
		

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

var reserved_for_picking := false
var reserved_for_dismantling := false:
	set(new_value):
		if new_value:
			$DismantleIndicator.show()
		else:
			$DismantleIndicator.hide()
		
		reserved_for_dismantling = new_value

var _initial_state: Variant

func initialize(_item_id: Items.Id, _amount: int = 1, state: ItemState = ItemState.Normal) -> ItemOnGround:
	item_id = _item_id
	$ItemAmount.amount = _amount
	$ItemAmount.amount_changed.connect(_amount_changed)
	
	$ProgressBar.value = build_progress
	
	#current_state = state
	_initial_state = state
		
	
	return self
	
func _amount_changed(new_amount: int) -> void:
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	item = Items.get_by_id(item_id) as Item
	
	current_durability = item.durability
	max_durability = item.durability

	sprite.visible = false
	var coordinates := Globals.get_map().global_position_to_coordinate(global_position)
	
	occluder.visible = false
	
	if item.rendering_type == Item.RenderingType.Sprite:
		sprite.visible = true
		sprite.texture = item.texture
		sprite.hframes = item.hframes
		sprite.vframes = item.vframes
		sprite.frame = item.frame
		sprite.centered = false
		var sprite_size := sprite.texture.get_size() / Vector2(sprite.hframes, sprite.vframes)
		sprite.offset = (- item.origin * sprite_size) - Vector2(MainMap.CELL_SIZE / 2)
		occluder.visible = item.cast_shadow_enabled
		occluder.position = (item.cast_shadow_origin * sprite_size)
		
	elif item.rendering_type == Item.RenderingType.Terrain:
		Events.terrain_placed.emit(coordinates, item.target_layer, item.terrain_set_id, item.terrain_id, item.is_solid, self)

	if item.scene:
		var scene := item.scene.instantiate() as Node2D
		scene.name = "scene"
		add_child(scene)
		item_scene = scene
		
	# Trigger current_state setter with initialized item
	print("Initializing item on ground, setting state: ", _initial_state)
	current_state = _initial_state
	Events.item_placed_on_ground.emit(self, global_position)

func _exit_tree() -> void:
	if item.rendering_type == Item.RenderingType.Terrain:
		Events.terrain_cleared.emit(Globals.get_map().global_position_to_coordinate(global_position), item.target_layer, item.terrain_set_id)
	Events.item_removed_from_ground.emit(self)

func reduce_durability(amount: float) -> void:
	current_durability -= amount

func has_durability_left() -> bool:
	return current_durability > 0

func generate_drops() -> void:
	print("Generate drops yeah?")
	for item_drop in item.item_drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_drop.item_id, amount)
			# TODO: Randomize position slightly
			new_item_on_ground.global_position = global_position
			get_parent().add_child(new_item_on_ground)


# Blueprint related

var finish_emitted := false
var build_progress := 0.0

func finish_construction() -> void:
	if not finish_emitted:
		#Events.solid_cell_placed.emit(Globals.get_map().global_position_to_coordinate(global_position))
		$Sprite2D.modulate = Color.WHITE
		$ProgressBar.hide()
		
		await get_tree().process_frame
				
		print("Finish construction")
		finish_emitted = true
		Events.construction_finished.emit(self)

func increase_build_progress(amount: float) -> void:
	build_progress += amount
	$ProgressBar.value = build_progress
	if build_progress > 0:
		if current_state != ItemState.Normal:
			current_state = ItemState.Normal

	if is_finished():
		finish_construction()

func is_finished() -> bool:
	return build_progress >= 1.0

func has_materials() -> bool:
	var material_requirements := Items.get_crafting_requirements(item_id)
	
	for requirement in material_requirements:
		var deposited := $Inventory.get_items().find(func(depo: Inventory.InventoryItemAmount) -> bool: return depo.id == requirement.item_id) as Inventory.InventoryItemAmount
		if deposited.amount < requirement.amount:
			return false
	
	return true
