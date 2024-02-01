extends Control

func _ready() -> void:
	Events.current_time_changed.connect(_current_time_changed)

func _current_time_changed(new_time: float) -> void:
	%IndicatorTexture.rotation_degrees = (new_time + 0.5) * 360
