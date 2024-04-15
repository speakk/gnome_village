class_name GrowthSpotComponent extends Component

signal plant_set(plant_component: PlantComponent)

var has_growing: bool

var growth_requirement_inventory: InventoryComponent
var plant_component: PlantComponent

func _init() -> void:
	id = Components.Id.GrowthSpot
	subscriptions.append(
		Subscription.new(id, Components.Id.Inventory,
		func(_inventory: InventoryComponent) -> void:
			growth_requirement_inventory = _inventory
			)
	)

func consume_growth_requirement(growth_requirement_item: EntityDefinition, amount: int) -> void:
	growth_requirement_inventory.remove_item_amount(growth_requirement_item, amount)

func increase_growth_requirement(growth_requirement_item: EntityDefinition, amount: int) -> void:
	growth_requirement_inventory.add_item_amount(growth_requirement_item, amount)

func start_growing_plant(plant: EntityDefinition) -> void:
	if not has_growing:
		var planted_plant := Entity.from_definition(plant)
		Events.request_entity_add.emit(planted_plant)
		WorldPositionComponent.set_world_position(planted_plant, get_owner().component_container.get_by_id(Components.Id.WorldPosition).current_position)
		plant_component = planted_plant.component_container.get_by_id(Components.Id.Plant)
		# TODO: This is so that the can be "dismantled". Do this any other way
		# in the future.
		planted_plant.component_container.add_component(ConstructableComponent.new())
		plant_component.grows_in = self
		plant_component.managed_by_player = true
		plant_set.emit(plant_component)
				
		has_growing = true

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["has_growing"] = has_growing
	if plant_component and is_instance_valid(plant_component.get_owner()):
		dict["plant_component_owner_id"] = SaveSystem.get_save_id(plant_component.get_owner())
		
	return dict

func deserialize(dict: Dictionary) -> void:
	super.deserialize(dict)
	has_growing = dict["has_growing"]
	if dict.has("plant_component_owner_id"):
		SaveSystem.queue_entity_reference_by_id(SaveSystem.EntityReferenceEntry.new(
			dict["plant_component_owner_id"], func(owner: Entity) -> void:
			plant_component = owner.component_container.get_by_id(Components.Id.Plant)
			))
	
#endregion
