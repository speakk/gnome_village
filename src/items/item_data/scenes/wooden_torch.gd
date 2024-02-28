extends Node3D

@onready var MODEL_SCENE := preload("res://assets/blender_models/wooden_torch.blend")

func _ready() -> void:
	$wooden_torch/Torch.cast_shadow = true

func set_blueprint(is_blueprint: bool) -> void:
	print("Set as blueprint: ", is_blueprint)
	if is_blueprint:
		Globals.apply_blueprint_material($wooden_torch)
	else:
		$wooden_torch.queue_free()
		var new_scene := MODEL_SCENE.instantiate()
		new_scene.name = "wooden_torch"
		add_child(new_scene)
		print("Added ye old")

func set_active(active: bool) -> void:
	if active:
		$OmniLight3D.visible = true
		$GPUParticles3D.visible = true
	else:
		$OmniLight3D.visible = false
		$GPUParticles3D.visible = false
