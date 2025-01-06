#scenes
"main_scene"
    FlexNode{ width:100vw height:100vh justify_main:SpaceEvenly justify_cross:FlexEnd  }
    FocusPolicy::Pass
    Picking::Ignore
    "text"
        BackgroundColor(#277777)
        TextLine{ text: "Hello, World!" }