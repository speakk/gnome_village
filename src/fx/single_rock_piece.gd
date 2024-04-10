class_name SingleRockPiece extends RigidBody3D

func set_custom_scale(new_scale: float) -> void:
	$MeshInstance3D.scale = Vector3(new_scale, new_scale, new_scale)
	var shape: BoxShape3D = $CollisionShape3D.shape.duplicate() as BoxShape3D
	var orig_size := shape.size.x
	shape.size = Vector3(orig_size * new_scale, orig_size * new_scale, orig_size * new_scale)
	$CollisionShape3D.shape = shape
