class_name DismantleTree extends Task

var item_on_ground: ItemOnGround

func _init(_item_on_ground: ItemOnGround) -> void:
	task_name = "Dismantle target"
	order_type = Task.OrderType.Sequence
	item_on_ground = _item_on_ground
	
	var task := DismantleTask.new({
		target = item_on_ground
	})
	
	register_subtask(task)

func _ready() -> void:	
	Events.dismantle_cancel_issued.connect(func(_item_on_ground: ItemOnGround) -> void:
		if _item_on_ground == item_on_ground:
			is_cancelled = true
	)
