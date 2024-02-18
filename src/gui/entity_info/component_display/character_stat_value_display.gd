class_name CharacterStatValueDisplay extends MarginContainer

func set_value(new_value: float) -> void:
	$Label.text = "%s %%" % [(new_value * 100)]
