class_name SmeltingJob extends Resource

@export var recipe: Recipe
@export var amount: int
@export var job_amount_type_id: JobAmountType.Id

var _generated_tasks: Array[Task]

static func create(_recipe: Recipe, _amount: int, _job_amount_type_id: JobAmountType.Id) -> SmeltingJob:
	var new_job := SmeltingJob.new()
	new_job.recipe = _recipe
	new_job.amount = _amount
	new_job.job_amount_type_id = _job_amount_type_id
	return new_job

func start(smelter: SmelterComponent) -> void:
	if job_amount_type_id == JobAmountType.Id.OneOff:
		for i in amount:
			var task := SmeltTree.new(smelter, recipe)
			TaskManager.add_task(task)
