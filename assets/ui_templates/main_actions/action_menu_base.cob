#import
colours as colours

#commands
LoadImages[ "textures/wooden_wall_ui_texture.png" ]

#defs
+action_menu = \
    GridNode{ width: 100% height: 100% }
    LoadedImageNode{image:"textures/wooden_wall_ui_texture.png" mode: Tiled{ tile_x: true tile_y: true stretch_value: 2.5 }}
    Splat<Padding>(10px)
    BrRadius(5px)
    Splat<Border>(4px)
    BorderColor($colours::primary_dark)

\