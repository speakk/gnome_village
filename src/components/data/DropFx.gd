class_name DropFxComponent extends Component

@export var effects: Array[Effect]

func _init() -> void:
	id = Components.Id.DropFx

func on_exit() -> void:
	for effect in effects:
		var scene: EffectScene = effect.effect_scene.instantiate()
		get_owner().get_parent_node_3d().add_child(scene)
		scene.global_position = get_owner().global_position
		scene.start()
		scene.finished.connect(func() -> void:
			scene.queue_free()
			)
			
