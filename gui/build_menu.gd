extends PanelContainer

func _ready() -> void:
	var constructable_ids := Items.get_constructable_item_ids()
	for child in %BuildOptions.get_children():
		child.queue_free()
		
	for item_id in constructable_ids:
		print("ID FOUND:", item_id)
		var item := Items.get_by_id(item_id)
		var button := Button.new()
		button.text = item.display_name
		%BuildOptions.add_child(button)
