extends MarginContainer

func _on_build_button_pressed() -> void:
	show_submenu(%BuildMenu, %BuildButton)

func _on_plant_button_pressed() -> void:
	show_submenu(%PlantMenu, %PlantButton)

func _on_order_button_pressed() -> void:
	show_submenu(%OrderMenu, %OrderButton)

func _on_zone_button_pressed() -> void:
	show_submenu(%ZoneMenu, %ZoneButton)

var orig_offset: Vector2

func show_submenu(menu: Node, button: Node) -> void:
	$ClickSoundPlayer.play()
	for child in %MainButtons.get_children():
		child.button_pressed = false
	var visibility := menu.visible as bool
	hide_submenu()
	if not visibility:
		menu.show()
		%SubMenuContainer.show()
		#orig_offset = %SubMenuContainer.position
		menu.position = Vector2()
		var tween := create_tween().bind_node(self)
		tween.tween_property(menu, "position", Vector2(0, 200), 0)
		tween.tween_property(menu, "position", Vector2(), 0.2).set_trans(Tween.TRANS_SPRING).set_ease(Tween.EASE_OUT)
		button.button_pressed = true

func hide_submenu() -> void:
	Events.ui.action_cleared.emit()
	%BuildMenu.hide()
	%PlantMenu.hide()
	%OrderMenu.hide()
	%ZoneMenu.hide()
	%SubMenuContainer.hide()

func _ready() -> void:
	for child in %MainButtons.get_children():
		child.button_pressed = false
	hide_submenu()
	
	Events.entity_selected.connect(func(_entity: Object) -> void: $ClickSoundPlayer.play())


func _process(delta: float) -> void:
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT):
		hide_submenu()
		Events.ui_action_selected.emit(null)
		Events.clear_entity_selections.emit()
