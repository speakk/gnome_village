extends Node3D

var cloud_speed_multiplier: float = 10

#func _physics_process(delta: float) -> void:
	#for child: Node3D in get_children():
		#child.global_position.x += delta * cloud_speed_multiplier
		#if child.global_position.x >= MainMap.MAP_SIZE_X / 2 + 3:
			#child.global_position.x = - MainMap.MAP_SIZE_X / 2 - 3
