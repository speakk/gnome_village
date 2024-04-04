class_name Cloud extends Node3D

var MODELS: Array[PackedScene] = [
	preload("res://assets/blender_models/clouds/cloud1.blend"),
	preload("res://assets/blender_models/clouds/cloud2.blend"),
	preload("res://assets/blender_models/clouds/cloud3.blend"),
	preload("res://assets/blender_models/clouds/cloud4.blend")
	]

var speed_multiplier := 1.0

func _ready() -> void:
	var model: Node3D = MODELS.pick_random().instantiate()
	add_child(model)
	
	var random_scale := randf_range(1.0, 3.5)
	model.scale = Vector3(random_scale, random_scale, random_scale)
	
	var model_child: MeshInstance3D = model.get_child(0)
	model_child.sorting_offset = 100
	model_child.transparency = 0.5
	#
	#var material: StandardMaterial3D = model_child.mesh.surface_get_material(0)
	#material.depth_draw_mode = BaseMaterial3D.DEPTH_DRAW_DISABLED
	
	#model_child.extra_cull_margin = 10
