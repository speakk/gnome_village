class_name SceneSystem extends System

func _ready() -> void:
	Events.construction_finished.connect(_construction_finished)

func _construction_finished(item: ItemOnGround) -> void:
	var container := item.component_container
	if container.has_component(Components.Id.Scene):
		container.get_by_id(Components.Id.Scene).set_active(true)
