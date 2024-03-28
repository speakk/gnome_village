extends Node3D

@onready var camera_3d: Camera3D = $Camera3D

@onready var new_game_button: MainMenuButton = %NewGameButton
@onready var options_button: MainMenuButton = %OptionsButton
@onready var quit_button: MainMenuButton = %QuitButton


func _ready() -> void:
	$sky.time_of_day = 0.327
	new_game_button.pressed.connect(func() -> void: Events.new_game_requested.emit())
	#new_game_button.pressed.connect(func() -> void: Events.scene_change_requested(SceneManager.SceneId.InGame))
	quit_button.pressed.connect(func() -> void:
		#Globals.quitting = true
		get_tree().quit()
		)

func _process(delta: float) -> void:
	var ray_result: Variant = _get_ray_result(get_viewport().get_mouse_position())
	if ray_result and ray_result.get_parent().has_method("on_hover"):
		ray_result.get_parent().on_hover()

@warning_ignore("untyped_declaration")
func _get_ray_result(hover_position: Vector2):
	var from := camera_3d.project_ray_origin(hover_position)
	var to := from + camera_3d.project_ray_normal(hover_position)*400
	var space_state := get_world_3d().direct_space_state
	var ray_query := PhysicsRayQueryParameters3D.create(from, to)
	ray_query.collide_with_areas = true
	ray_query.collide_with_bodies = false
	#ray_query.collision_mask = 0b00000000_00000000_00000000_00000010
	var raycast_result := space_state.intersect_ray(ray_query)
	#print("full result", raycast_result)
	if raycast_result.has("collider"):
		return raycast_result.get("collider") as Area3D
