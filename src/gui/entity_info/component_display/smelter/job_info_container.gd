class_name JobInfoContainer extends HBoxContainer

func set_job(job: SmeltingJob) -> void:
	%RecipeLabel.text = job.recipe.produces[0].item.display_name
	%AmountLabel.text = "%s" % job.amount
	%AmountTypeLabel.text = JobAmountType.get_label(job.job_amount_type_id)
