extends Entity

var MODEL_SCENE := preload("res://assets/blender_models/wooden_torch.blend")

var _scene: Node3D

func _ready() -> void:
	super._ready()
	_scene = MODEL_SCENE.instantiate()
	#_scene.get_node("Torch").cast_shadow = true
	add_child(_scene)

func set_blueprint(is_blueprint: bool) -> void:
	print("Set as blueprint: ", is_blueprint)
	if _scene:
		if has_node("wooden_torch"):
			$wooden_torch.queue_free()
		_scene.queue_free()
		
	_scene = MODEL_SCENE.instantiate()
		
	if is_blueprint:
		Globals.apply_blueprint_material(_scene)

	add_child(_scene)

func set_active(active: bool) -> void:
	if active:
		$OmniLight3D.visible = true
		$GPUParticles3D.visible = true
	else:
		$OmniLight3D.visible = false
		$GPUParticles3D.visible = false
