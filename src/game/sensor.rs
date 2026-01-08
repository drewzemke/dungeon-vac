use bevy_egui::egui::WidgetText;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sensor {
    HitWall,
    SpaceLeft,
    SpaceRight,
}

// FIXME: should this live somewhere else? it isn't purely about game logic
impl From<Sensor> for String {
    fn from(val: Sensor) -> Self {
        match val {
            Sensor::HitWall => "WHEN hit wall",
            Sensor::SpaceLeft => "WHEN space left",
            Sensor::SpaceRight => "WHEN space right",
        }
        .into()
    }
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl From<Sensor> for WidgetText {
    fn from(val: Sensor) -> Self {
        String::from(val).into()
    }
}
