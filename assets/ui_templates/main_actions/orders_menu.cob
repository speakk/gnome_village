#import
action_menu_base as action_menu_base
colours as colours

#scenes
"orders_menu"
    +action_menu_base::action_menu{}

    "order_items"
        GridNode{ grid_auto_flow: Column}

"order_item"
    GridNode{}
    Interactive{}
    ControlRoot

    "label"
        GridNode{}
        TextLine{ text:"order_item_label" size:25 font:{family: "m5x7" } justify: Center}
        TextLineColor($colours::primary)
        ControlMember
        Animated<TextLineColor>{
                    idle:Hsla{hue:45 saturation:1.0 lightness:1.0 alpha:1.0}
                    hover:Hsla{hue:45 saturation:1.0 lightness:0.8 alpha:1.0}
                    hover_with:{ duration:0.75 ease:OutExpo delay:0.00 }
                    press_with:{ duration:0.75 ease:OutExpo delay:0.01 }
                }