extends VBoxContainer

var smelter: SmelterComponent

var recipe_selector_id_map := {}

func _ready() -> void:
	%AmountTypeSelector.clear()
	%RecipeSelector.item_selected.connect(_recipe_selected)
	
	for amount_type: int in JobAmountType.Id.values():
		%AmountTypeSelector.add_item(JobAmountType.get_label(amount_type), amount_type)

func _recipe_selected(index: int) -> void:
	var recipe: Recipe = recipe_selector_id_map[index]
	var cost_text := "Cost: "
	for i in recipe.requires.size():
		var item_requirement: ItemRequirement = recipe.requires[i]
		cost_text += "(%s x %s)" % [item_requirement.amount, item_requirement.item.display_name]
		if i < recipe.requires.size() - 1:
			cost_text += ", "
			
	%CostLineEdit.text = cost_text

func set_smelter(_smelter: SmelterComponent) -> void:
	smelter = _smelter
	%RecipeSelector.clear()
	
	recipe_selector_id_map = {}
	var recipe_index: int = 0
	for recipe in smelter.supported_recipes:
		var produces := recipe.produces
		var label := produces[0].item.display_name
		%RecipeSelector.add_item(label, recipe_index)
		recipe_selector_id_map[recipe_index] = recipe
		recipe_index += 1
	
	%RecipeSelector.select(0)

func _on_add_job_button_pressed() -> void:
	var smelting_job := SmeltingJob.create(
		recipe_selector_id_map[%RecipeSelector.selected],
		%AmountSpinBox.value,
		%AmountTypeSelector.selected
	)
	
	smelter.add_job(smelting_job)
