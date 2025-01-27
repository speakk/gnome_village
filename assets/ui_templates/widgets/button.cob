#import
builtin.colors.tailwind as tw

#commands
LoadImages[ "ui_templates/pngs/button.png" ]

#defs

+main_bottom_button = \
    GridNode{ justify_lines: Center margin: { right: 10px }}
    ControlRoot

    "image"
        GridNode{ width: 100px grid_row:{ start: 1 } grid_column:{ start: 1}}
        LoadedImageNode{image:"ui_templates/pngs/button.png" }
        FocusPolicy::Pass
        Picking::Ignore

        Animated<ImageNodeColor>{
            idle:Hsla{hue:45 saturation:0.0 lightness:0.8 alpha:1.0}
            hover:Hsla{hue:45 saturation:0.0 lightness:1.0 alpha:1.0}
            press:Hsla{hue:45 saturation:1.0 lightness:0.9 alpha:1.0}
            hover_with:{ duration:0.75 ease:OutExpo delay:0.00 }
            press_with:{ duration:0.75 ease:OutExpo delay:0.01 }
        }

    "text"
        GridNode{ justify_self_cross: Center grid_row:{ start: 1 } grid_column:{ start: 1} margin: { left: Auto right: Auto}}
        TextLine{ text:"" size:24 font:{family: "ThaleahFat" } justify: Center}
        ControlMember
        FocusPolicy::Pass
        Picking::Ignore
        Animated<TextLineColor>{
            idle:Hsla{hue:45 saturation:1.0 lightness:1.0 alpha:1.0}
            hover:Hsla{hue:45 saturation:1.0 lightness:0.8 alpha:1.0}
            hover_with:{ duration:0.75 ease:OutExpo delay:0.00 }
            press_with:{ duration:0.75 ease:OutExpo delay:0.01 }
        }

\

#scenes
"button"
    +main_bottom_button{}
    Interactive{}
