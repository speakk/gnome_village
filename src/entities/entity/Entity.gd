class_name Entity extends Node3D

@onready var component_container: ComponentContainer = $ComponentContainer

var default_components: Array[Component] = [
	SelectableComponent.new(),
	DisplayNameComponent.new(),
	WorldPositionComponent.new(),
	ItemAmountComponent.new(),
]

var show_amount_number := true
var _should_set_components := true

var definition: EntityDefinition:
	set(new_definition):
		definition = new_definition

func _amount_changed(new_amount: int) -> void:
	if show_amount_number:
		$ItemAmountLabel.text = "%s" % new_amount
		if new_amount > 1:
			$ItemAmountLabel.show()
		else:
			$ItemAmountLabel.hide()
	
	if new_amount <= 0:
		queue_free()

func _ready() -> void:
	set_notify_transform(true)
	if _should_set_components:
		set_item_components()
	#var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
	#item_amount.amount_changed.connect(func(new_amount: int) -> void:
		#if new_amount > 1:
			#$ItemAmountLabel.text = new_amount
			#$ItemAmountLabel.show()
		#else:
			#$ItemAmountLabel.hide()
		#
		#if new_amount <= 0:
			#queue_free()
		#)

func _exit_tree() -> void:
	Events.item_removed_from_ground.emit(self)

func set_item_components() -> void:
	if definition:
		for default_component: Component in default_components:
			component_container.add_component(default_component)
			
		for component: Component in definition.components:
			component_container.add_component(component)

		var display_name_component: DisplayNameComponent = component_container.get_by_id(Components.Id.DisplayName)
		if display_name_component:
			display_name_component.display_name = definition.display_name

		var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
		if item_amount:
			item_amount.item = definition
			if item_amount.amount == 0:
				item_amount.amount = 1

static func from_definition(entity_definition: EntityDefinition) -> Entity:
	var custom_scene_component: Variant = entity_definition.get_component_by_id(Components.Id.Scene)
	var scene: Node3D
	if custom_scene_component:
		var custom_scene: Node3D = custom_scene_component.scene.instantiate()
		if custom_scene is Entity:
			scene = custom_scene
		else:
			scene = load("res://src/entities/entity/Entity.tscn").instantiate()
			scene.add_child(custom_scene)
	else:
		scene = load("res://src/entities/entity/Entity.tscn").instantiate()

	scene.definition = entity_definition
	
	return scene

func serialize() -> Dictionary:
	var dict := {}
	if definition:
		dict["definition"] = definition.serialize()
	dict["component_container"] = component_container.serialize()
	dict["scene_path"] = scene_file_path
	dict["save_id"] = SaveSystem.get_save_id(self)
	
	return dict

static func static_deserialize(parent: Node, dict: Dictionary) -> Entity:
	var entity: Entity
	
	# TODO: Do we need this logic like this?
	if dict.has("definition"):
		entity = from_definition(EntityDefinition.deserialize(dict["definition"]))
	else:
		entity = load(dict["scene_path"]).instantiate()
	
	entity._should_set_components = false
	
	parent.add_child(entity)
	
	entity.component_container.component_owner = entity
	entity.component_container.deserialize(dict["component_container"])
	entity.set_meta("save_id", dict["save_id"])
	entity.deserialize(dict)
	SaveSystem.register_entity_reference(entity)
	return entity

func deserialize(dict: Dictionary) -> void:
	pass
