extends MarginContainer

var smelter: SmelterComponent

func set_component(_smelter: SmelterComponent) -> void:
	smelter = _smelter
	%AddJobSection.set_smelter(smelter)
	%ExistingJobsSection.set_smelter(smelter)
