class_name SceneManager extends Node

@onready var IN_GAME := preload("res://src/main_3d.tscn")
@onready var MENU := preload("res://src/gui/main_menu/main_menu.tscn")

enum SceneId {
	Menu, InGame
}

func _ready() -> void:
	Events.scene_change_requested.connect(_scene_change_requested)
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
