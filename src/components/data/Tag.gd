class_name TagComponent extends Component

@export var tags: Array[Tag]

enum Tag {
	Tree, PlayerMade, Rock
}

func _init() -> void:
	id = Components.Id.Tag

func add_tag(tag: Tag) -> void:
	if tags.has(tag):
		return
	
	tags.append(tag)

func has_tag(tag: Tag) -> bool:
	return tags.has(tag)
