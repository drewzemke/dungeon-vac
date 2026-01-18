use bevy::prelude::*;

#[derive(Message)]
pub struct ResetLevel;

#[derive(Message)]
pub struct LevelComplete;

#[derive(Message)]
pub struct LoadLevel(pub usize);

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LevelComplete>()
            .add_message::<ResetLevel>()
            .add_message::<LoadLevel>();
    }
}
