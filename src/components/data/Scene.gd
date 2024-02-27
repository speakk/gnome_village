class_name SceneComponent extends Component

@export var scene: PackedScene

var _instantiated_scene: Node

func _init() -> void:
	id = Components.Id.Scene

func on_enter() -> void:
	_instantiated_scene = scene.instantiate()
	get_owner().add_child(_instantiated_scene)

func on_exit() -> void:
	get_owner().call_deferred("remove_child", _instantiated_scene)
