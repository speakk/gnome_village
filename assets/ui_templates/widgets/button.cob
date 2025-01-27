#import
builtin.colors.tailwind as tw

#commands
LoadFonts[ "ThaleahFat" ]

#defs
$NORMAL_BUTTON = $tw::INDIGO_400
$HOVERED_BUTTON = $tw::INDIGO_300
$PRESSED_BUTTON = $tw::BLUE_400
$BORDER_BUTTON = $tw::INDIGO_900
$BORDER_DISPLAY = $tw::SKY_950
// Defaults to 'button' styling.
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

#scenes
"button"
    +main_bottom_button{}