extends MarginContainer

func _on_build_button_pressed() -> void:
	show_submenu(%BuildMenu, %BuildButton)

func _on_plant_button_pressed() -> void:
	show_submenu(%PlantMenu, %PlantButton)

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
	%SubMenuContainer.hide()
