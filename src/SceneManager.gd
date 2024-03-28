class_name SceneManager extends Node

@onready var IN_GAME := preload("res://src/in_game.tscn")
@onready var MENU := preload("res://src/gui/main_menu/main_menu.tscn")

enum SceneId {
	Menu, InGame
}

var _current_scene_id: SceneId

func _ready() -> void:
	Events.scene_change_requested.connect(_scene_change_requested)
	Events.new_game_requested.connect(func() -> void:
		_scene_change_requested(SceneId.InGame)
		await get_tree().physics_frame
		(get_child(0) as InGame).new_game()
		)
	var default_scene := MENU.instantiate()
	add_child(default_scene)

func _scene_change_requested(new_scene_id: SceneId) -> void:
	for child in get_children():
		child.queue_free()
	
	match new_scene_id:
		SceneId.Menu:
			add_child(MENU.instantiate())
		SceneId.InGame:
			add_child(IN_GAME.instantiate())
	
	_current_scene_id = new_scene_id

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("quicksave"):
		if _current_scene_id == SceneId.InGame:
			var in_game: InGame = get_child(0)
			in_game.save_game()
	
	if Input.is_action_just_pressed("quickload"):
		_scene_change_requested(SceneId.InGame)
		await get_tree().physics_frame
		(get_child(0) as InGame).quick_load()
		
		
