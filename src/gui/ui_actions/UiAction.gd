class_name UiAction extends RefCounted

enum UiActionId {
	None, Build, Dismantle, ZoneAddTiles, ZoneRemoveTiles
}

static var UiActionIdMap: Dictionary = {
	UiActionId.None: null,
	UiActionId.Build: Build,
}

class Build extends UiAction:
	var item_id: Items.Id
	func _init(_item_id: Items.Id) -> void:
		item_id = _item_id

class ZoneAddTiles extends UiAction:
	var zone: Zone
	func _init(_zone: Zone) -> void:
		zone = _zone

class Dismantle extends UiAction:
	pass
