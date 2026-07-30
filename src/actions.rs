#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

#[derive(Clone, Copy)]
pub enum Action {
    Exit,
    Close,
    ToggleBar,
    ToggleFloating,

    Move(Direction),

    Resize(Direction, u16),

    Fullscreen,

    Workspace(u8),
    MoveToWorkspace(u8),

    Focus(Direction),

    VolumeUp,
    VolumeDown,
    Mute,

    Run(&'static str),
}
