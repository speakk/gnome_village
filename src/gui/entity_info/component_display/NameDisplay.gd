extends MarginContainer

func set_component(component: Component) -> void:
	%NameLabel.text = (component as DisplayNameComponent).display_name
