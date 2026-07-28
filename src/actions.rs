#[derive(Clone, Copy)]
pub enum Action {
    Exit,
    Close,
    ToggleBar,
    ToggleFloating,

    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,

    IncreaseWidth,
    DecreaseWidth,
    IncreaseHeight,
    DecreaseHeight,

    Fullscreen,

    Workspace(u8),
    MoveToWorkspace(u8),

    FocusLeft,
    FocusRight,

    VolumeUp,
    VolumeDown,
    Mute,

    Run(&'static str),
}
