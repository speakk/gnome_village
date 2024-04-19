class_name SolidVisualizer extends Node3D

@onready var multi_mesh_instance_3d: MultiMeshInstance3D = $MultiMeshInstance3D

func _ready() -> void:
	Events.debug.toggle_solid_visualizer.connect(func() -> void:
		visible = !visible
		if visible:
			_refresh()
		)

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("toggle_solid_visualizer"):
		Events.debug.toggle_solid_visualizer.emit()

func _refresh() -> void:
	var multi_mesh := multi_mesh_instance_3d.multimesh
	var transforms: Array[Transform3D]
	
	var solid_coordinates := PathFinder.get_all_solid()
	
	for coordinate in solid_coordinates:
		var pos := Globals.get_map().coordinate_to_global_position(coordinate)
		var point_transform := Transform3D(Basis(), pos)
		transforms.append(point_transform)
				
	multi_mesh.instance_count = transforms.size()
	
	for i in multi_mesh.instance_count:
		multi_mesh.set_instance_transform(i, transforms[i])
	
