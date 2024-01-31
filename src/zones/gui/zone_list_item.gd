class_name ZoneListItem extends HBoxContainer

var zone: Zone

signal focused

func set_zone(_zone: Zone) -> void:
	zone = _zone
	%Label.text = zone.zone_name

func _on_delete_button_pressed() -> void:
	Events.zone_delete_pressed.emit(zone)

func _on_add_area_button_button_down() -> void:
	var ui_action := UiAction.ZoneAddTiles.new(zone)
	Events.ui_action_selected.emit(ui_action)
	
	focused.emit()
	#Events.player_action_selected.emit(Globals.PlayerAction.ZoneAddTiles, {
		#zone: zone
	#})

func _on_remove_area_button_pressed() -> void:
	pass # Replace with function body.

func _on_select_button_pressed() -> void:
	focused.emit()

func select() -> void:
	$SelectButton.text = "Selected [x]"
	
func deselect() -> void:
	$SelectButton.text = "Select"
