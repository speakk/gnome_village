extends Node

@export var container: ComponentContainer:
	set(new_value):
		container = new_value
		container.component_added.connect(func(component: Component) -> void:
			if component.id == Components.Id.CharacterStats:
				stats = component
			)
		
var stats: CharacterStatsComponent

func get_hunger() -> float:
	return stats.get_stat(CharacterStatsComponent.Id.Hunger).value

# TODO: Perhaps get some kind of task urgency score
func get_work() -> float:
	return 0.3

func get_tiredness() -> float:
	return stats.get_stat(CharacterStatsComponent.Id.Tiredness).value

func get_melancholy() -> float:
	return stats.get_stat(CharacterStatsComponent.Id.Melancholy).value
