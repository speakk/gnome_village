class_name DropComponent extends Component

var ENTITY := load("res://src/items/entity/Entity.tscn")

@export var drops: Array[ItemDrop]

func _init() -> void:
	id = Components.Id.Drop

func on_exit() -> void:
	for item_drop in drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_entity := ENTITY.instantiate() as Entity
			# TODO: Randomize position slightly
			Events.request_entity_add.emit(new_entity)
			new_entity.definition = item_drop.item
			WorldPositionComponent.set_world_position(new_entity, get_owner().global_position)
