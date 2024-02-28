class_name SceneComponent extends Component

@export var scene: PackedScene

var _instantiated_scene: Node

func _init() -> void:
	id = Components.Id.Scene

func on_enter() -> void:
	_instantiated_scene = scene.instantiate()
	get_owner().add_child(_instantiated_scene)
	set_active(false)

func on_exit() -> void:
	get_owner().call_deferred("remove_child", _instantiated_scene)

func set_active(active: bool) -> void:
	if _instantiated_scene.has_method("set_active"):
		_instantiated_scene.set_active(active)

func set_blueprint(is_blueprint: bool) -> void:
	if _instantiated_scene.has_method("set_blueprint"):
		_instantiated_scene.set_blueprint(is_blueprint)
