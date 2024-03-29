class_name EntitySelectionDraw extends Node3D

@export var entity_selection_mesh: Mesh

var selected_entities: Array[Node3D]

func _ready() -> void:
	Events.entity_selected.connect(_entity_selected)
	Events.entity_deselected.connect(_entity_deselected)
	Events.clear_entity_selections.connect(_clear_entity_selections)

func _clear_entity_selections() -> void:
	selected_entities.clear()
	redraw()

func _entity_selected(entity: Node3D) -> void:
	selected_entities.append(entity)
	redraw()

func _entity_deselected(entity: Node3D) -> void:
	selected_entities.erase(entity)
	redraw()

func redraw() -> void:
	for child in get_children():
		child.queue_free()
	
	for entity in selected_entities:
		var mesh_instance := MeshInstance3D.new()
		mesh_instance.mesh = entity_selection_mesh
		add_child(mesh_instance)
		mesh_instance.global_position = entity.global_position
		mesh_instance.global_position.y = 0.1
