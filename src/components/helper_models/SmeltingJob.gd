class_name SmeltingJob extends Resource

@export var recipe: Recipe
@export var amount: int
@export var job_amount_type_id: JobAmountType.Id

signal finished
var is_finished := false

var smelter: SmelterComponent

# One off
var _amount_left: int

var _generated_tasks: Array[Task]

static func create(_recipe: Recipe, _amount: int, _job_amount_type_id: JobAmountType.Id) -> SmeltingJob:
	var new_job := SmeltingJob.new()
	new_job.recipe = _recipe
	new_job.amount = _amount
	new_job.job_amount_type_id = _job_amount_type_id
	return new_job

func start(_smelter: SmelterComponent) -> void:
	smelter = _smelter
	match job_amount_type_id:
		JobAmountType.Id.OneOff:
			_amount_left = amount
	
	_generate_next_task()
			#for i in amount:
				#var task := SmeltTree.new(smelter, recipe)
				#TaskManager.add_task(task)

func _generate_next_task() -> void:
	match job_amount_type_id:
		JobAmountType.Id.OneOff:
			if _amount_left > 0:
				var task := SmeltTree.new(smelter, recipe)
				TaskManager.add_task(task)
				task.finished.connect(func() -> void:
					_amount_left -= 1
					_generate_next_task()
					)
			else:
				finished.emit()
				is_finished = true
