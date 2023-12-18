extends Node

signal blueprint_placed(tile_position: Vector2i, blueprint: Blueprint)
signal blueprint_finished(blueprint: Blueprint)

signal task_finished(task: Task)
signal task_assigner_finished(task_assigner: Variant)

signal map_ready(map: MainMap)
