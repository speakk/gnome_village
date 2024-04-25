class_name JobAmountType extends Resource

enum Id {
	OneOff,
	EnsureStocksHave
}

static var display_strings := {
	Id.OneOff: "Make once",
	Id.EnsureStocksHave: "Ensure stocks have X",
}

static func get_label(id: Id) -> String:
	return display_strings[id] as String
