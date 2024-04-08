extends MarginContainer

func _on_dismantle_button_pressed() -> void:
	var ui_action := UiAction.Dismantle.new([Components.Id.PlayerMade])
	Events.ui_action_selected.emit(ui_action)

func _on_chop_trees_button_pressed() -> void:
	var ui_action := UiAction.Dismantle.new([Components.Id.Tree])
	Events.ui_action_selected.emit(ui_action)
