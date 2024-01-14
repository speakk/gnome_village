class_name OpenDoorActorAction extends ActorAction

var door: ItemOnGround

func initialize(params: Variant) -> ActorAction:
	door = params.door
	return self

func process_action(actor: Settler, delta: float) -> void:
	door.item_scene.open_by_amount(actor.open_door_speed * delta)
	if door.item_scene.is_open():
		finished.emit(self)
