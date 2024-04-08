extends MarginContainer

func _on_dismantle_button_pressed() -> void:
	var ui_action := UiAction.Dismantle.new([TagComponent.Tag.PlayerMade])
	Events.ui_action_selected.emit(ui_action)

func _on_chop_trees_button_pressed() -> void:
	var ui_action := UiAction.Dismantle.new([TagComponent.Tag.Tree])
	Events.ui_action_selected.emit(ui_action)


func _on_mine_button_pressed() -> void:
	var ui_action := UiAction.Dismantle.new([TagComponent.Tag.Rock])
	Events.ui_action_selected.emit(ui_action)
