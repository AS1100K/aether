use azalea::Vec3;

/// This represents configuration of AntiAFK Plugin
/// All the enabled settings are triggered randomly
#[derive(Clone)]
pub struct AntiAFKConfig {
    /// 25% Chance of jump being executed after (`random_head_rotation` i.e. 50% chance).
    pub jump: bool,
    /// 25% Chance of sneak being executed after (`random_head_rotation` i.e. 50% chance)
    pub sneak: bool,
    /// If enabled, this will make the bot move around `central_afk_location`, there is 12.5% chance of this executing.
    /// If disabled, `flip_lever` will be executed if available otherwise `Swing` packet will be sent.
    /// `Walk::None` represents no walking is allowed
    pub walk: Walk,
    /// Distance from the `central_afk_location` bot should be moving
    /// Default to 4
    pub walk_distance: Option<u8>,
    /// Location around which the bot should revolve around
    /// Default is current position i.e. if `None` is provided
    pub central_afk_location: Option<Vec3>,
    /// Flips the lever (if within range)
    pub flip_lever: bool
}

#[derive(Default, Clone, Eq, PartialEq)]
pub enum Walk {
    #[default]
    SafeWalk,
    Walk,
    None
}