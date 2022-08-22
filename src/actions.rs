use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlatformerAction {
    Right,
    Left,
    Down,
    Up,

    Jump,
    Ability,
    Dash,
    Dodge,
    Pause,
}
