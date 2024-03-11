class_name ConsumableComponent extends Component

@export var provides: Array[Satisfaction] 

func _init() -> void:
	id = Components.Id.Consumable
	groups = [Groups.Id.Consumable]

func consume() -> Array[Satisfaction]:
	get_owner().queue_free()
	return provides.duplicate()
