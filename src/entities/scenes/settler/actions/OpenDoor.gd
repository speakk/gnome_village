class_name OpenDoorActorAction extends ActorAction

var door: DoorComponent

func _init(actor: Settler, params: Dictionary) -> void:
	super._init(actor)
	door = params.door

func process_action(delta: float) -> void:
	door.open_by_amount(actor.open_door_speed * delta)
	if door.is_open():
		finished.emit()
