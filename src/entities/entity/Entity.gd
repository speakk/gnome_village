class_name Entity extends Node3D

@onready var component_container: ComponentContainer = $ComponentContainer

var show_amount_number := true

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
		for component: Component in definition.components:
			component_container.add_component(component)

		# This untyped display_name_component is here because of an OBSCURE bug
		# If you type display_name_component here, basically all components become invalid
		# and everything breaks
		@warning_ignore("untyped_declaration")
		var display_name_component = component_container.get_by_id(Components.Id.DisplayName)
		display_name_component.display_name = definition.display_name

		var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
		item_amount.item = definition

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

static func deserialize(parent: Node, dict: Dictionary) -> Entity:
	var entity: Entity
	
	# TODO: Do we need this logic like this?
	if dict.has("definition"):
		entity = from_definition(EntityDefinition.deserialize(dict["definition"]))
	else:
		entity = load(dict["scene_path"]).instantiate()
	
	parent.add_child(entity)
	
	entity.component_container.component_owner = entity
	entity.component_container.deserialize(dict["component_container"])
	entity.set_meta("save_id", dict["save_id"])
	SaveSystem.register_entity_reference(entity)
	return entity
