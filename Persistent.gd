class_name Persistent extends Node

var save_id: int = -1
	
func get_save_id() -> int:
	if save_id < 0:
		save_id = SaveSystem.get_next_save_id()
	
	return save_id

func set_save_id(new_id: int) -> void:
	save_id = new_id
