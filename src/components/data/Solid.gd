class_name SolidComponent extends Component

func on_enter() -> void:
	Events.solid_cell_placed.emit(get_container().get_by_id(Components.Id.WorldPosition).coordinate)

func on_exit() -> void:
	Events.solid_cell_removed.emit(get_container().get_by_id(Components.Id.WorldPosition).coordinate)
