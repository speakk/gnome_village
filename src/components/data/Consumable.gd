class_name ConsumableComponent extends Component

@export var provides: Array[Satisfaction]

# TODO: Make the task system validate if identical task already exists
var reserved := false

func _init() -> void:
	id = Components.Id.Consumable
	#groups = [Groups.Id.Consumable]

func consume() -> Array[Satisfaction]:
	# TODO: Something going wrong here to need to check
	if is_instance_valid(get_owner()):
		get_owner().queue_free()
	return provides.duplicate()

func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["reserved"] = reserved
	dict["provides"] = provides.map(func(provides: Satisfaction) -> Dictionary:
		return provides.serialize()
		)
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	reserved = dict["reserved"]
	provides.assign(dict["provides"].map(func(satisfaction_dict: Dictionary) -> Satisfaction:
		var satisfaction := Satisfaction.new()
		satisfaction.deserialize(satisfaction_dict)
		return satisfaction
		))
	
