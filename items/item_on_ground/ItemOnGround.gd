extends Node2D

class_name ItemOnGround

@onready var sprite := $Sprite2D as Sprite2D
@onready var occluder := $LightOccluder2D as LightOccluder2D
@onready var itemAmount := $ItemAmount as ItemAmount

var item_id: Items.Id
var item: Item


var reserved_for_picking := false

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
