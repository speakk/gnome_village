extends Node

@export var bird_chance: float = 0.0015

func _physics_process(delta: float) -> void:
	if randf() < bird_chance:
		$Birds.get_children().pick_random().play()

func _ready() -> void:
	$Music/Ambient5.play()
