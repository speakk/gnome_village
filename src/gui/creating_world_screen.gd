extends PanelContainer

func _ready() -> void:
	hide()
	Events.world_creation.begin.connect(_creating_world)
	Events.world_creation.finished.connect(_creating_world_finished)

	Events.world_creation.ground_and_ocean.connect(func() -> void: _set_current_action_text("Ground and the ocean"))
	Events.world_creation.rocks.connect(func() -> void: _set_current_action_text("Rock formations"))
	Events.world_creation.rivers.connect(func() -> void: _set_current_action_text("Rivers"))
	Events.world_creation.grass.connect(func() -> void: _set_current_action_text("Grass"))
	Events.world_creation.entities.connect(func() -> void: _set_current_action_text("Entities"))
	
	%CurrentActionLabel.text = ""

func _set_current_action_text(text: String) -> void:
	%CurrentActionLabel.text = "Creating: %s" % text

func _creating_world() -> void:
	process_mode = PROCESS_MODE_ALWAYS
	mouse_filter = MOUSE_FILTER_STOP
	show()

func _creating_world_finished() -> void:
	process_mode = PROCESS_MODE_DISABLED
	mouse_filter = MOUSE_FILTER_PASS
	hide()
