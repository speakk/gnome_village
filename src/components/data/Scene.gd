class_name SceneComponent extends Component

@export var scene: PackedScene
@export var custom_subscriptions: Array[StringSubscription]:
	set(new_value):
		custom_subscriptions = new_value
		subscriptions.append_array(custom_subscriptions.map(func(custom_subscription: StringSubscription) -> Subscription:
			return Subscription.new(id, custom_subscription.target_id, func(component: Component) -> void:
				_instantiated_scene.call(custom_subscription.method_name, component)
				)
		))

var _instantiated_scene: Node

func _init() -> void:
	id = Components.Id.Scene
	subscriptions.append_array([
		Subscription.new(self.id, Components.Id.Blueprint, func (blueprint: BlueprintComponent) -> void:
			set_blueprint(true)
			blueprint.removed.connect(func() -> void: set_blueprint(false))
			),
		Subscription.new(self.id, Components.Id.Constructable, func (constructable: ConstructableComponent) -> void:
			set_active(false)
			constructable.finished.connect(func() -> void:
				set_active(true)
				)
			),
	])

func on_enter() -> void:
	_instantiated_scene = scene.instantiate()
	get_owner().add_child(_instantiated_scene)
	set_active(false)

func on_exit() -> void:
	super.on_exit()
	get_owner().call_deferred("remove_child", _instantiated_scene)

func set_active(active: bool) -> void:
	if _instantiated_scene.has_method("set_active"):
		_instantiated_scene.set_active(active)

func set_blueprint(is_blueprint: bool) -> void:
	if _instantiated_scene:
		if _instantiated_scene.has_method("set_blueprint"):
			_instantiated_scene.set_blueprint(is_blueprint)
