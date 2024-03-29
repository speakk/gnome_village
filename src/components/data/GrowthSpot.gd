class_name GrowthSpotComponent extends Component

signal plant_set(new_plant: EntityDefinition)

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

func start_growing_plant(item: EntityDefinition) -> void:
	if not has_growing:
		plant_set.emit(item)
		# TODO: Just implement get_by_id for components as well to avoid this? (already in component_container)
		var components := item.components
		for component in components:
			if component.id == Components.Id.Plant:
				plant_component = component
				break
				
		has_growing = true

#region Serialization
func serialize() -> Dictionary:
	var dict := super.serialize()
	dict["has_growing"] = has_growing
	if plant_component:
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
