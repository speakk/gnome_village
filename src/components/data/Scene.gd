class_name SceneComponent extends Component

@export var scene: PackedScene
@export var custom_subscriptions: Array[StringSubscription]:
	set(new_value):
		custom_subscriptions = new_value
		subscriptions.assign(custom_subscriptions.map(func(custom_subscription: StringSubscription) -> Subscription:
			print("Are we even here?")
			return Subscription.new(id, custom_subscription.target_id, func(component: Component) -> void:
				print("CALL???")
				_instantiated_scene.call(custom_subscription.method_name, component)
				)
		))

var _instantiated_scene: Node

func _init() -> void:
	id = Components.Id.Scene
	#
	#print("Custom subs size? ", custom_subscriptions.size())
	#subscriptions.append_array(custom_subscriptions.map(func(custom_subscription: StringSubscription) -> Subscription:
		#print("Are we even here?")
		#return Subscription.new(id, custom_subscription.target_id, func(component: Component) -> void:
			#print("CALL???")
			#_instantiated_scene.call(custom_subscription.method_name, component)
			#)
	#))
	#print("Subscriptions now??", subscriptions.size())

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
