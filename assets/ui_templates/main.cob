#scenes
"main_scene"
    FlexNode{ width:100vw height:100vh justify_main:SpaceEvenly justify_cross:FlexEnd  }
    FocusPolicy::Pass
    Picking::Ignore

    "buttons_container"
        FlexNode{ width:100vw height:Auto justify_main:FlexStart justify_cross:FlexEnd  }
        Margin{ left: 35px }
        FocusPolicy::Pass
        Picking::Ignore

    "action_menu_container"
        AbsoluteGridNode{ width: 200px height: 300px bottom: 90px left: 90px }
        FocusPolicy::Pass
        Picking::Ignore