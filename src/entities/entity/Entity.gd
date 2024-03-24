class_name Entity extends Node3D

#var ENTITY := load("res://src/entities/entity/Entity.tscn")

@onready var component_container: ComponentContainer = $ComponentContainer

var show_amount_number := true

var definition: EntityDefinition:
	set(new_definition):
		definition = new_definition

func save() -> Dictionary:
	var save_dict := {
		"position_x" = global_position.x,
		"position_y" = global_position.y,
	}
	
	return save_dict

func load_save(save_dict: Dictionary) -> void:
	global_position.x = save_dict["position_x"]
	global_position.y = save_dict["position_y"]

	Events.item_placed_on_ground.emit(self, global_position)

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
	var item_amount: ItemAmountComponent = component_container.get_by_id(Components.Id.ItemAmount)
	item_amount.amount_changed.connect(func(new_amount: int) -> void:
		if new_amount > 1:
			$ItemAmountLabel.text = new_amount
			$ItemAmountLabel.show()
		else:
			$ItemAmountLabel.hide()
		
		if new_amount <= 0:
			queue_free()
		)
	

func _exit_tree() -> void:
	Events.item_removed_from_ground.emit(self)

func place_at_coordinate(coordinate: Vector2i) -> void:
	var new_position := Globals.get_map().coordinate_to_global_position(coordinate)
	WorldPositionComponent.set_world_position(self, new_position)

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
		#item_amount.amount = 1

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
