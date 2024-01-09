class_name OpenDoorActorAction extends ActorAction

var door: ItemOnGround

func initialize(params: Variant) -> ActorAction:
	door = params.door
	return self

func process_action(actor: Settler, delta: float) -> void:
	door.scene.open_by_amount(actor.open_door_speed * delta)
	if door.scene.is_fully_open():
		finished.emit(self)
