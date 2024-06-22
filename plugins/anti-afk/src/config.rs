/// This represents configuration of AntiAFK Plugin
/// All the enabled settings are triggered randomly
#[derive(Clone)]
pub struct AntiAFKConfig {
    /// 25% Chance of jump being executed after (`random_head_rotation` i.e. 50% chance).
    pub jump: bool,
    /// 25% Chance of sneak being executed after (`random_head_rotation` i.e. 50% chance)
    pub sneak: bool,
    /// If enabled, this will make the bot randomly move.
    /// This is experimental, and is not very safe
    pub walk: bool,
    /// Flips the lever (if within range)
    pub flip_lever: bool
}