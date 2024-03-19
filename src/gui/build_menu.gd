extends MarginContainer

func _ready() -> void:
	var constructable_items := Items.get_constructable_items()
	for child in %BuildOptions.get_children():
		child.queue_free()
		
	for item in constructable_items:
		var button := Button.new()
		button.text = item.display_name
		button.pressed.connect(_construction_button_pressed.bind(item))
		%BuildOptions.add_child(button)

func _construction_button_pressed(item: Item) -> void:
	var ui_action := UiAction.Build.new(item)
	Events.ui_action_selected.emit(ui_action)
