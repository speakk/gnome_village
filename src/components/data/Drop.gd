class_name DropComponent extends Component

var ITEM_ON_GROUND := load("res://src/items/item_on_ground/ItemOnGround.tscn")

@export var drops: Array[ItemDrop]

func _init() -> void:
	id = Components.Id.Drop

func on_exit() -> void:
	for item_drop in drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_item_on_ground := ITEM_ON_GROUND.instantiate() as ItemOnGround
			# TODO: Randomize position slightly
			Events.request_entity_add.emit(new_item_on_ground)
			new_item_on_ground.item = item_drop.item
			WorldPositionComponent.set_world_position(new_item_on_ground, get_owner().global_position)
