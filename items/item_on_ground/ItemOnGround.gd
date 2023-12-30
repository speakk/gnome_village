extends Node2D

class_name ItemOnGround

@onready var ITEM_ON_GROUND := preload("res://items/item_on_ground/ItemOnGround.tscn")

@onready var sprite := $Sprite2D as Sprite2D
@onready var occluder := $LightOccluder2D as LightOccluder2D
@onready var itemAmount := $ItemAmount as ItemAmount

var item_id: Items.Id
var item: Item

var current_durability: float = 10

var reserved_for_picking := false
var reserved_for_dismantling := false

func initialize(_item_id: Items.Id, _amount: int = 1) -> ItemOnGround:
	item_id = _item_id
	$ItemAmount.amount = _amount
	$ItemAmount.amount_changed.connect(_amount_changed)
	
	return self
	
func _amount_changed(new_amount: int) -> void:
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	item = Items.get_by_id(item_id) as Item
	
	current_durability = item.durability

	sprite.visible = false
	var coordinates := Globals.get_map().global_position_to_coordinate(global_position)
	
	if item.is_solid:
		Events.solid_cell_placed.emit(coordinates)
	
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
		Events.terrain_placed.emit(coordinates, item.target_layer, item.terrain_set_id, item.terrain_id, item.is_solid)

func reduce_durability(amount: float) -> void:
	current_durability -= amount

func has_durability_left() -> bool:
	return current_durability > 0

func generate_drops() -> void:
	for item_drop in item.item_drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_item_on_ground := (ITEM_ON_GROUND.instantiate() as ItemOnGround).initialize(item_drop.item_id, amount)
			# TODO: Randomize position slightly
			new_item_on_ground.global_position = global_position
			get_parent().add_child(new_item_on_ground)
