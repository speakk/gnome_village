#import
builtin.colors.tailwind as tw

#commands
LoadImages[ "ui_templates/pngs/button.png" ]

#defs
$NORMAL_BUTTON = $tw::INDIGO_400
$HOVERED_BUTTON = $tw::INDIGO_300
$PRESSED_BUTTON = $tw::BLUE_400
$BORDER_BUTTON = $tw::INDIGO_900
$BORDER_DISPLAY = $tw::SKY_950
// Defaults to 'button' styling.
/*
+main_bottom_button = \
    FlexNode{ width:100px height:Auto justify_main:Center }
    Splat<Border>(2px)
    Splat<Padding>(5px)
    Splat<Margin>(5px)
    BrRadius(10px)
    BorderColor($BORDER_BUTTON)
    Responsive<BackgroundColor>{ idle:$NORMAL_BUTTON hover:$HOVERED_BUTTON press:$PRESSED_BUTTON }

    "text"
        FlexNode
        TextLine{ text:"" size:30 font:{family: "ThaleahFat" width:Normal style:Normal weight:Normal} }
\
*/

/*

+main_bottom_button = \
    FlexNode{ width: Auto justify_main:Center justify_cross: Center }

    "image"
        FlexNode{ width: 200px }
        LoadedImageNode{image:"ui_templates/pngs/button.png" }

    "text"
        AbsoluteGridNode{ justify_main:Center justify_cross: Center }
        TextLine{ text:"" size:30 font:{family: "ThaleahFat" } }

\

*/

+main_bottom_button = \
    GridNode{ justify_lines: Center margin: { right: 10px }}

    "image"
        GridNode{ width: 130px grid_row:{ start: 1 } grid_column:{ start: 1}}
        LoadedImageNode{image:"ui_templates/pngs/button.png" }

    "text"
        GridNode{ justify_self_cross: Center grid_row:{ start: 1 } grid_column:{ start: 1} margin: { left: Auto right: Auto}}
        TextLine{ text:"" size:30 font:{family: "ThaleahFat" } justify: Center}

\

#scenes
"button"
    +main_bottom_button{}
