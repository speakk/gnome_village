class_name DropFxComponent extends Component

@export var effects: Array[Effect]

func _init() -> void:
	id = Components.Id.DropFx

func on_exit() -> void:
	for effect in effects:
		var effect_entity := Entity.new()
		Events.request_entity_add.emit(effect_entity)
		var container: ComponentContainer = effect_entity.component_container
		var scene_component: SceneComponent = container.add_component(SceneComponent.new(effect.effect_scene))
		var position_component: WorldPositionComponent = effect_entity.component_container.add_component(WorldPositionComponent.new())
		position_component.current_position = get_container().get_by_id(Components.Id.WorldPosition).current_position
		var scene: EffectScene = scene_component.get_scene()
		scene.start()
		scene.finished.connect(func() -> void:
			effect_entity.delete()
			)
			
