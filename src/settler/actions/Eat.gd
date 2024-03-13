class_name EatActorAction extends ActorAction

var consumable: ConsumableComponent
var eating_time: float = 2.0

func _init(actor: Settler, params: Dictionary) -> void:
	super._init(actor, params)
	consumable = params.consumable

func process_action(delta: float) -> void:
	if consumable:
		eating_time -= delta
		if eating_time <= 0:
			var character_stats: CharacterStatsComponent = actor.component_container.get_by_id(Components.Id.CharacterStats)
			character_stats.apply_consumable(consumable.consume())
