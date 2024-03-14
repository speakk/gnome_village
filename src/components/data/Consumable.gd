class_name ConsumableComponent extends Component

@export var provides: Array[Satisfaction] 

# TODO: Make the task system validate if identical task already exists
var reserved := false

func _init() -> void:
	id = Components.Id.Consumable
	groups = [Groups.Id.Consumable]

func consume() -> Array[Satisfaction]:
	# TODO: Something going wrong here to need to check
	if is_instance_valid(get_owner()):
		get_owner().queue_free()
	return provides.duplicate()
