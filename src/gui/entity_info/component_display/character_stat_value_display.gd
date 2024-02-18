class_name CharacterStatValueDisplay extends MarginContainer

var stat: CharacterStatsComponent.CharacterStat

func _process(delta: float) -> void:
	if stat:
		var new_value := stat.value
		$Label.text = "%.0f %%" % [(new_value * 100)]
		$TextureProgressBar.value = new_value

func set_stat(_stat: CharacterStatsComponent.CharacterStat) -> void:
	stat = _stat

