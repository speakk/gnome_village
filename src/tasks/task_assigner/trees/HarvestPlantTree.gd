class_name HarvestPlantTree extends TaskTree

var DISMANTLE_TASK := preload("res://src/tasks/task_data/Dismantle.tscn")

var _plant: PlantComponent

func _init() -> void:
	task_name = "Harvest plant"

func _ready() -> void:
	name = "HarvestPlantTree"

func finish_tree() -> void:
	print("FINISH HARVEST")
	clean_up()

func initialize(plant: PlantComponent) -> HarvestPlantTree:
	order_type = TaskTreeBranch.OrderType.Sequence
	_plant = plant
	
	var task := DISMANTLE_TASK.instantiate() as DismantleTask
	task.initialize({
		target = plant.get_owner()
	})
	
	var dismantle_leaf := TaskTreeLeaf.new()
	dismantle_leaf.set_task(task)
	dismantle_leaf.name = "Dismantle_leaf"
	
	add_child(dismantle_leaf)
	
	return self

func _handle_task_failure(task: Task) -> void:
	print("Harvest failed")
