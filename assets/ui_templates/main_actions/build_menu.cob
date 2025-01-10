#import
action_menu_base as action_menu_base
colours as colours

#scenes
"build_menu"
    +action_menu_base::action_menu{}

    "categories"
        GridNode{}

    "build_items"
        GridNode{ grid_auto_flow: Column}

"build_item"
    GridNode{}

    "label"
        GridNode{}
        TextLine{ text:"build_item_label" size:25 font:{family: "m5x7" } justify: Center}
        TextLineColor($colours::primary)