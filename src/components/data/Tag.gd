class_name TagComponent extends Component

@export var tags: Array[Tag]

enum Tag {
	Tree, PlayerMade, Rock, Wood, CopperOre
}

func _init() -> void:
	id = Components.Id.Tag

func add_tag(tag: Tag) -> void:
	if tags.has(tag):
		return
	
	tags.append(tag)

func has_tag(tag: Tag) -> bool:
	return tags.has(tag)

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["tags"] = tags
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	if dict.has("tags"):
		tags = dict["tags"]
#endregion
