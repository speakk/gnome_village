class_name SmelterExistingJobsSection extends MarginContainer

var JOB_INFO_CONTAINER := preload("res://src/gui/entity_info/component_display/smelter/job_info_container.tscn")

func _ready() -> void:
	for child in %JobList.get_children():
		child.queue_free()

func set_smelter(smelter: SmelterComponent) -> void:
	smelter.job_added.connect(_job_added)

func _job_added(job: SmeltingJob) -> void:
	var job_info_container := JOB_INFO_CONTAINER.instantiate()
	job_info_container.set_job(job)
	%JobList.add_child(job_info_container)
