class_name DismantleIndicatorComponent extends Component

var SCENE := preload("res://src/components/data/scenes/DismantleIndicator.tscn")

func _init() -> void:
	id = Components.Id.DismantleIndicator

func on_enter() -> void:
	var scene := SCENE.instantiate()
	var scene_component: SceneComponent = get_container().get_by_id(Components.Id.Scene)
	if scene_component:
		scene_component.add_child(scene)
	else:
		scene_component = get_container().add_component(SceneComponent.new())
	
	scene_component.add_child(scene)
