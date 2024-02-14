class_name GrowthSpot extends Node

var provides_growth_requirements: Array[GrowthRequirementAmount]

func consume_growth_requirement(growth_requirement_id: GrowthRequirement.Id, amount: int) -> void:
	var provides: GrowthRequirementAmount = provides_growth_requirements.filter(func(a: GrowthRequirementAmount) -> bool:
		return a.growth_requirement_id == growth_requirement_id).front()
	
	provides.amount -= amount
	if provides.amount <= 0:
		provides_growth_requirements.erase(provides)

func increase_growth_requirement(growth_requirement_id: GrowthRequirement.Id, amount: int) -> void:
	var provides_array: Array[GrowthRequirementAmount] = provides_growth_requirements.filter(func(a: GrowthRequirementAmount) -> bool:
		return a.growth_requirement_id == growth_requirement_id)
	
	var provides: GrowthRequirementAmount
	
	if provides_array.size() == 0:
		provides = GrowthRequirementAmount.new()
		provides.growth_requirement_id = growth_requirement_id
		provides_growth_requirements.append(provides)
	else:
		provides = provides_array.front()
	
	provides.amount += amount
