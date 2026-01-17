use bevy::prelude::*;

#[derive(Message)]
pub struct ResetLevel;

#[derive(Message)]
pub struct LevelComplete;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LevelComplete>()
            .add_message::<ResetLevel>();
    }
}
