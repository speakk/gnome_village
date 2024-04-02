class_name DropComponent extends Component

var ENTITY := load("res://src/entities/entity/Entity.tscn")

@export var drops: Array[ItemDrop]

func _init() -> void:
	id = Components.Id.Drop

func on_exit() -> void:
	for item_drop in drops:
		if randf() <= item_drop.probability:
			var amount := randi_range(item_drop.amount_min, item_drop.amount_max)
			var new_entity := Entity.from_definition(item_drop.item)
			# TODO: Randomize position slightly
			Events.request_entity_add.emit(new_entity)
			WorldPositionComponent.set_world_position(new_entity, get_owner().global_position)

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["drops"] = drops.map(func(drop: ItemDrop) -> Dictionary:
		return drop.serialize()
		)
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	drops.assign(dict["drops"].map(func(drop_dict: Dictionary) -> ItemDrop:
		var drop := ItemDrop.new()
		drop.deserialize(drop_dict)
		return drop
		))
#endregion
