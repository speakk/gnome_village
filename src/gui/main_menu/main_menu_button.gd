@tool
extends Node3D

var hovered := false

@export var original_color: Color
@export var hover_color: Color

@export var text: String:
	set(new_value):
		%Label3D.text = new_value
		text = new_value


func _ready() -> void:
	%button/Cube.material_override = %button/Cube.get_active_material(0).duplicate()
	%button/Cube.get_active_material(0).albedo_color = original_color

func on_hover() -> void:
	hovered = true


func _on_area_3d_input_event(camera: Node, event: InputEvent, position: Vector3, normal: Vector3, shape_idx: int) -> void:
	#print("EVENT", event)
	pass


func _on_area_3d_mouse_entered() -> void:
	hovered = true
	var material: StandardMaterial3D = %button/Cube.get_active_material(0)
	material.albedo_color = hover_color
	$ClickSoundPlayer.play()
	
	await create_tween().tween_property($Node3D, "rotation_degrees", Vector3(0, -10, 0), 0.1).finished
	create_tween().tween_property($Node3D, "rotation_degrees", Vector3(0, 0, 0), 0.5).set_trans(Tween.TRANS_BOUNCE).set_ease(Tween.EASE_OUT)


func _on_area_3d_mouse_exited() -> void:
	hovered = false
	var material: StandardMaterial3D = %button/Cube.get_active_material(0)
	material.albedo_color = original_color
