#import
button as button

#scenes
"main_menu"
    FlexNode{ width:100vw height:100vh justify_main:SpaceEvenly justify_cross:FlexEnd  }
    FocusPolicy::Pass
    Picking::Ignore

    "buttons_container"
        FlexNode{ width:100vw height:Auto justify_main:FlexStart justify_cross:FlexEnd  }
        Margin{ left: 35px }
        FocusPolicy::Pass
        Picking::Ignore

    "action_menu_container"
        AbsoluteGridNode{ width: 200px height: 300px bottom: 60px left: 35px top: Auto }
        FocusPolicy::Pass
        Picking::Ignore