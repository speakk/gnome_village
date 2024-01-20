extends Node2D

class_name ItemOnGround

enum ItemState {
	Blueprint, Normal
}

@onready var persistent := $Persistent as Persistent

@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")

@onready var sprite := $Sprite2D as Sprite2D
@onready var occluder := $LightOccluder2D as LightOccluder2D
@onready var itemAmount := $ItemAmount as ItemAmount



var item_scene: Node2D

var item: Item

var item_id: Items.Id:
	set(new_item_id):
		item_id = new_item_id
		item = Items.get_by_id(item_id) as Item
		current_durability = item.durability
		max_durability = item.durability

		if not sprite:
			item_id = new_item_id
			return
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
			if item_scene:
				remove_child(get_node("scene"))
				item_scene.queue_free()
				
			var scene := item.scene.instantiate() as Node2D
			scene.name = "scene"
			add_child(scene)
			item_scene = scene
		



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
	max_durability = save_dict["max_durability"]
	current_state = save_dict["current_state"]
	item_id = save_dict["item_id"]
	_initial_state = current_state
	item = Items.get_by_id(item_id) as Item

func initialize(_item_id: Items.Id, _amount: int = 1, state: ItemState = ItemState.Normal) -> ItemOnGround:
	item_id = _item_id
	$ItemAmount.amount = _amount
	$ItemAmount.amount_changed.connect(_amount_changed)
	
	current_state = state
	_initial_state = state
		
	print("Did we call init here?", _initial_state)
	
	return self
	
func _amount_changed(new_amount: int) -> void:
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	# Trigger current_state setter with initialized item
	print("Initializing item on ground, setting state: ", _initial_state, current_state)
	#current_state = _initial_state
	var tmp_item_id := item_id
	item_id = tmp_item_id
	print("Setting item id", item_id)
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
