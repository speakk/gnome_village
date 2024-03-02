extends Node3D

@onready var original_rotation := rotation_degrees
@onready var mesh_instance_3d: MeshInstance3D = $MeshInstance3D

const WOODEN_DOOR_MESH := preload("res://src/items/item_data/scenes/extra_meshes/wooden_door_mesh.tres")

func correct_orientation() -> void:
	print("Correcting orientation")
	if not mesh_instance_3d:
		return
		
	print("Actually doing it")
	var self_coordinate := (Globals.get_map() as MainMap3D).global_position_to_coordinate(get_parent_node_3d().global_position)
	var surrounding_coordinates := PathFinder.get_surrounding_coordinates(self_coordinate, false)
	for coordinate in surrounding_coordinates:
		var coordinate_entities := Globals.get_map().get_map_entities(coordinate)
		for coordinate_entity in coordinate_entities:
			print("Entity in surrounding")
			if coordinate_entity.component_container.has_component(Components.Id.Constructable):
				if (coordinate_entity.component_container.get_by_id(Components.Id.Constructable) as ConstructableComponent).solid_when_started:
					var angle_to := Vector2(self_coordinate).angle_to_point(Vector2(coordinate))
					if is_equal_approx(angle_to, 2*PI) or is_equal_approx(angle_to, 0):
						mesh_instance_3d.rotation_degrees = Vector3(0, 0, 0)
						mesh_instance_3d.position.x = 0.5
						mesh_instance_3d.position.z = 0
						position.x = -0.5
						position.z = 0
					else:
						mesh_instance_3d.rotation_degrees = Vector3(0, 90, 0)
						position.x = 0
						position.z = -0.5
						mesh_instance_3d.position.z = 0.5
						mesh_instance_3d.position.x = 0
						
					return

func _ready() -> void:
	correct_orientation()
	Events.map_changed.connect(_map_changed)

# TODO: Some kind of bigger cells/quadtree instead of literally
# doing this for every door on the whole map
func _map_changed(coordinate: Vector2i) -> void:
	if get_parent_node_3d().global_position.distance_to(Globals.get_map().coordinate_to_global_position(coordinate)) < MainMap3D.CELL_SIZE.x * 2:
		correct_orientation()

func set_door(door_component: DoorComponent) -> void:
	door_component.open_amount_changed.connect(func(new_amount: float) -> void:
			rotation_degrees = Vector3(0, original_rotation.y + new_amount * 90, 0)
			)
	
func set_blueprint(is_blueprint: bool) -> void:
	if is_blueprint:
		Globals.apply_blueprint_material(self)
	else:
		if not has_node("MeshInstance3D"):
			return
			
		var original_transform: Transform3D = mesh_instance_3d.transform
		mesh_instance_3d.queue_free()
		var new_scene := MeshInstance3D.new()
		new_scene.name = "MeshInstance3D"
		new_scene.mesh = WOODEN_DOOR_MESH
		add_child(new_scene)
		new_scene.transform = original_transform
		mesh_instance_3d = new_scene
