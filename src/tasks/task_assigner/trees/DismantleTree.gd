extends TaskTree

class_name DismantleTree

var DISMANTLE_TASK := preload("res://src/tasks/task_data/Dismantle.tscn")

var item_on_ground: ItemOnGround

func _init() -> void:
	task_name = "Dismantle target"

func _ready() -> void:
	name = "Dismantle_Tree"
	
	Events.dismantle_finished.connect(func(_item_on_ground: ItemOnGround) -> void:
		if _item_on_ground == item_on_ground:
			clean_up()
	)
	
	Events.dismantle_cancel_issued.connect(func(_item_on_ground: ItemOnGround) -> void:
		if _item_on_ground == item_on_ground:
			clean_up()
	)
	
func initialize(_item_on_ground: ItemOnGround) -> DismantleTree:
	order_type = TaskTreeBranch.OrderType.Sequence
	item_on_ground = _item_on_ground
	
	var task := DISMANTLE_TASK.instantiate() as DismantleTask
	task.initialize({
		target = item_on_ground
	})
	
	var dismantle_leaf := TaskTreeLeaf.new()
	dismantle_leaf.set_task(task)
	dismantle_leaf.name = "Dismantle_leaf"
	
	add_child(dismantle_leaf)

	return self
