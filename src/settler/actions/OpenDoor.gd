class_name OpenDoorActorAction extends ActorAction

var door: DoorComponent

func initialize(params: Variant) -> ActorAction:
	door = params.door
	return self

func process_action(actor: Settler, delta: float) -> void:
	door.open_by_amount(actor.open_door_speed * delta)
	if door.is_open():
		finished.emit(self)
