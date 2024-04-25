extends VBoxContainer

var smelter: SmelterComponent

var recipe_selector_id_map := {}

func _ready() -> void:
	%AmountTypeSelector.clear()
	
	for amount_type: int in JobAmountType.Id.values():
		%AmountTypeSelector.add_item(JobAmountType.get_label(amount_type), amount_type)

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

func _on_add_job_button_pressed() -> void:
	pass # Replace with function body.
