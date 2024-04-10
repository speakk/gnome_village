extends EffectScene

@onready var ROCK := preload("res://src/fx/single_rock_piece.tscn")

var amount_of_rocks: int = 15
var launch_speed_multiplier: float = 3
var lifetime: float = 3

func start() -> void:
	super.start()
	
	for i in amount_of_rocks:
		var rock: SingleRockPiece = ROCK.instantiate()
		add_child(rock)
		var rand_scale: float = randf_range(0.5, 2)
		rock.set_custom_scale(rand_scale)
		rock.scale = Vector3(rand_scale,rand_scale,rand_scale)
		rock.position = Vector3(
			randf_range(-0.5, 0.5),
			randf_range(0.5, 1),
			randf_range(-0.5, 0.5)
			)
		
		rock.apply_impulse(Vector3(
			randf_range(-1, 1),
			randf_range(0, 1),
			randf_range(-1, 1),
		) * launch_speed_multiplier, Vector3(
			randf_range(-0.5, 0.5),
			randf_range(-0.5, 0.5),
			randf_range(-0.5, 0.5),
		))
		
	await get_tree().create_timer(lifetime).timeout
	
	finished.emit()
