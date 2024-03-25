extends Node

var map: MainMap

#var quitting := false

func _ready() -> void:
	Events.map_ready.connect(_map_ready)

func _map_ready(_map: MainMap) -> void:
	map = _map

func get_map() -> MainMap:
	return map

var control_has_focus: bool = false

func register_focus_input(input: Control) -> void:
	input.focus_entered.connect(func() -> void: control_has_focus = true)
	input.focus_exited.connect(func() -> void: control_has_focus = false)
	
func is_focus_in_control() -> bool:
	return control_has_focus


static func weighted_random(weights: Array[float]) -> int:
	var weights_sum := 0.0
	for weight in weights:
		weights_sum += weight
	
	var remaining_distance := randf() * weights_sum
	for i in weights.size():
		remaining_distance -= weights[i]
		if remaining_distance < 0:
			return i
	
	return 0

func truncate_vec3(vector: Vector3) -> Vector2:
	return Vector2(vector.x, vector.z)

func extend_vec2(vector: Vector2) -> Vector3:
	return Vector3(vector.x, 0, vector.y)

func truncate_vec3i(vector: Vector3i) -> Vector2i:
	return Vector2i(vector.x, vector.z)

func extend_vec2i(vector: Vector2i) -> Vector3i:
	return Vector3i(vector.x, 0, vector.y)

func apply_blueprint_material(scene: Node3D) -> void:
	var meshes := scene.find_children("*","MeshInstance3D")
	for mesh_instance: MeshInstance3D in meshes:
		var mesh: Mesh = mesh_instance.mesh.duplicate(true)
		mesh_instance.mesh = mesh
		for surface in mesh.get_surface_count():
			var orig_prev := mesh.surface_get_material(surface)
			var prev := orig_prev.duplicate() as StandardMaterial3D
			mesh.surface_set_material(surface, prev)
			prev.albedo_color = Color(0.4, 0.5, 1.0, 0.4)
			prev.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA

var next_pass := preload("res://src/misc/blueprint_shader.tres") as ShaderMaterial
#func apply_blueprint_shader(scene: Node3D) -> void:
	#var meshes := scene.find_children("*","MeshInstance3D")
	#for mesh_instance: MeshInstance3D in meshes:
		#var mesh: Mesh = mesh_instance.mesh
		#for surface in mesh.get_surface_count():
			#print("Doing the thing")
			#var orig_prev := mesh.surface_get_material(surface)
			#var prev := orig_prev.duplicate()
			#mesh.surface_set_material(surface, prev)
			#var next: Material = prev.next_pass
			#while next:
				#prev = next
				#next = next.next_pass
			#if prev == next_pass:
				#continue
			#print("Assigning next pass")
			#prev.next_pass = next_pass

func clear_blueprint_shader(scene: Node3D) -> void:
	var meshes := scene.find_children("*","MeshInstance3D")
	for mesh_instance: MeshInstance3D in meshes:
		var mesh: Mesh = mesh_instance.mesh
		for surface in mesh.get_surface_count():
			print("Doing the thing")
			var prev := mesh.surface_get_material(surface)
			var next := prev.next_pass
			while next:
				if next == next_pass:
					next.next_pass = null
					print("Set next pass as null??")
				prev = next
				next = next.next_pass
				
