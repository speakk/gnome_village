extends MarginContainer

func _on_build_button_pressed() -> void:
	show_submenu(%BuildMenu, %BuildButton)

func _on_plant_button_pressed() -> void:
	show_submenu(%PlantMenu, %PlantButton)

func _on_order_button_pressed() -> void:
	show_submenu(%OrderMenu, %OrderButton)

func _on_zone_button_pressed() -> void:
	show_submenu(%ZoneMenu, %ZoneButton)

func show_submenu(menu: Node, button: Node) -> void:
	for child in %MainButtons.get_children():
		child.button_pressed = false
	var visibility := menu.visible as bool
	hide_submenu()
	if not visibility:
		menu.show()
		%SubMenuContainer.show()
		button.button_pressed = true

func hide_submenu() -> void:
	%BuildMenu.hide()
	%PlantMenu.hide()
	%OrderMenu.hide()
	%ZoneMenu.hide()
	%SubMenuContainer.hide()

func _on_dismantle_button_pressed() -> void:
	Events.dismantle_selected.emit()

func _ready() -> void:
	for child in %MainButtons.get_children():
		child.button_pressed = false
	hide_submenu()

