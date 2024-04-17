class_name ChildrenComponent extends Component

var _children: Array[Entity]

func _init() -> void:
	id = Components.Id.Children

func add_child(entity: Entity, inherit_transform: bool = true, delete_on_parent_delete: bool = true) -> void:
	var parent_component: ParentComponent = ParentComponent.new()
	parent_component.parent = get_owner()
	parent_component.inherit_transform = inherit_transform
	parent_component.delete_on_parent_delete = delete_on_parent_delete
	entity.component_container.add_component(parent_component)
	entity.delete_called.connect(func() -> void:
		_children.erase(entity)
		)
	_children.append(entity)
