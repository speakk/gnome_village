class_name SmeltingJob extends Resource

@export var recipe: Recipe
@export var amount: int
@export var job_amount_type: JobAmountType

static func create(_recipe: Recipe, _amount: int, _job_amount_type: JobAmountType) -> SmeltingJob:
	var new_job := SmeltingJob.new()
	new_job.recipe = _recipe
	new_job.amount = _amount
	new_job.job_amount_type = _job_amount_type
	return new_job
