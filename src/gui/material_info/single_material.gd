extends HBoxContainer

func set_type_amount(type_amount: SingleCategoryView.TypeAmount) -> void:
	var material_name: String = type_amount.definition.display_name
	
	$Description.text = "%s: %s" % [material_name, type_amount.amount]
