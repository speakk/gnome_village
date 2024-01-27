extends TaskTree

class_name DismantleTree

var DISMANTLE_TASK := preload("res://src/tasks/task_actuators/dismantle.tscn")

var item_on_ground: ItemOnGround

func _ready() -> void:
	name = "Dismantle_Tree"
	
	Events.dismantle_finished.connect(func(_item_on_ground: ItemOnGround) -> void:
		if _item_on_ground == item_on_ground:
			clean_up()
			# TODO: I don't like the tree calling queue_free
			item_on_ground.generate_drops() # TODO: DEFINITELY not the place for this
			item_on_ground.call_deferred("queue_free")
	)
	
	Events.dismantle_cancel_issued.connect(func(_item_on_ground: ItemOnGround) -> void:
		if _item_on_ground == item_on_ground:
			clean_up()
	)
	
func initialize(_item_on_ground: ItemOnGround) -> DismantleTree:
	order_type = TaskTreeBranch.OrderType.Sequence
	item_on_ground = _item_on_ground
	
	var dismantle_leaf := TaskTreeLeaf.new()
	dismantle_leaf.task = (DISMANTLE_TASK.instantiate() as DismantleTask).initialize(item_on_ground)
	dismantle_leaf.name = "Dismantle_leaf"
	
	add_child(dismantle_leaf)

	return self
