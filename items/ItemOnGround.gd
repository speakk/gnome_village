extends Node2D

class_name ItemOnGround

@onready var sprite := $Sprite2D as Sprite2D
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
	item = Items.get_by_id(item_id)
	sprite.texture = item.texture
	sprite.hframes = item.hframes
	sprite.vframes = item.vframes
	sprite.frame = item.frame
