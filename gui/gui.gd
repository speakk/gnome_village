extends MarginContainer

func _ready() -> void:
	pass
	# TODO: Loop through "buildables" and "plantables" or however you deal with this

func _on_build_button_pressed() -> void:
	show_submenu(%BuildMenu)

func _on_plant_button_pressed() -> void:
	show_submenu(%PlantMenu)

func show_submenu(menu: Node) -> void:
	var visibility := menu.visible as bool
	hide_submenu()
	if not visibility:
		menu.show()
		%SubMenuContainer.show()

func hide_submenu() -> void:
	%BuildMenu.hide()
	%PlantMenu.hide()
	%SubMenuContainer.hide()
