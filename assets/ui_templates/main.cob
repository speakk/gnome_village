
#scenes
"main_scene"
    FlexNode{ width:100vw height:100vh justify_main:SpaceEvenly justify_cross:FlexEnd  }
    FocusPolicy::Pass
    Picking::Ignore

    "buttons_container"
        FlexNode{ width:100vw height:Auto justify_main:FlexStart justify_cross:FlexEnd  }
        Splat<Margin>(5px)
        FocusPolicy::Pass
        Picking::Ignore
