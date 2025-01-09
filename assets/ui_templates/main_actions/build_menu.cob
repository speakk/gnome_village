#scenes
"build_menu"
    +action_menu_base

    "categories"
        GridNode{}

    "build_items"
        GridNode{ grid_auto_flow: Column}

"build_item"
    GridNode{}

    "label"
        GridNode{}
        TextLine{ text:"build_item_label" size:15 font:{family: "m5x7" } justify: Center}