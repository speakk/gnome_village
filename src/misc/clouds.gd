extends Node3D

@onready var CLOUD := preload("res://src/misc/cloud.tscn")

@export_range(0.1, 10, 0.1) var cloud_speed_multiplier: float = 10
@export_range(0.01, 1.0, 0.01) var cloud_chance: float = 0.01

var _cloud_timer_interval := 0.2
var _cloud_timer := _cloud_timer_interval

func spawn_cloud() -> void:
	var cloud: Cloud = CLOUD.instantiate()
	add_child(cloud)
	cloud.speed_multiplier = randf_range(0.6, 1.4)
	cloud.position = Vector3(
		-MainMap.MAP_SIZE_X / 2 - 10,
		randf_range(-1, 1),
		randf_range(-MainMap.MAP_SIZE_Y/2, MainMap.MAP_SIZE_Y/2))

func _physics_process(delta: float) -> void:
	_cloud_timer -= delta
	if _cloud_timer <= 0:
		_cloud_timer = _cloud_timer_interval
		if randf() < cloud_chance:
			spawn_cloud()
	
	for child: Node3D in get_children():
		if child.visible:
			child.global_position.x += delta * cloud_speed_multiplier * child.speed_multiplier
			if child.global_position.x >= MainMap.MAP_SIZE_X / 2 + 3:
				child.queue_free()
