class_name SceneComponent extends Component

@export var scene: PackedScene
@export var custom_subscriptions: Array[StringSubscription]:
	set(new_value):
		custom_subscriptions.assign(new_value)
		if custom_subscriptions.size() > 0:
			subscriptions.append_array(custom_subscriptions.map(func(custom_subscription: StringSubscription) -> Subscription:
				return Subscription.new(id, custom_subscription.target_id, func(component: Component) -> void:
					get_owner().call(custom_subscription.method_name, component)
					)
			))

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
	#_instantiated_scene = scene.instantiate()
	#get_owner().add_child(_instantiated_scene)
	set_active(false)

func on_exit() -> void:
	super.on_exit()
	#get_owner().call_deferred("remove_child", _instantiated_scene)

func set_active(active: bool) -> void:
	if get_owner().has_method("set_active"):
	#if _instantiated_scene.has_method("set_active"):
		get_owner().set_active(active)

func set_blueprint(is_blueprint: bool) -> void:
	if get_owner().has_method("set_blueprint"):
	#if _instantiated_scene:
		#if _instantiated_scene.has_method("set_blueprint"):
		get_owner().set_blueprint(is_blueprint)
