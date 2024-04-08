class_name UiAction extends RefCounted

var ui_action_id: UiActionId

enum UiActionId {
	None, Build, Dismantle, ZoneAddTiles, ZoneRemoveTiles
}

static var UiActionIdMap: Dictionary = {
	UiActionId.None: null,
	UiActionId.Build: Build,
}

class Build extends UiAction:
	var item: EntityDefinition
	func _init(_item: EntityDefinition = null) -> void:
		item = _item
		ui_action_id = UiActionId.Build

class ZoneAddTiles extends UiAction:
	var zone: Zone
	func _init(_zone: Zone) -> void:
		zone = _zone
		ui_action_id = UiActionId.ZoneAddTiles

class Dismantle extends UiAction:
	var target_tag_filters: Array[TagComponent.Tag]
	func _init(_target_tag_filters: Array[TagComponent.Tag]) -> void:
		ui_action_id = UiActionId.Dismantle
		target_tag_filters = _target_tag_filters
