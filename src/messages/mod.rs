use bevy::prelude::*;

mod system;
mod user;

pub use system::*;
pub use user::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LevelComplete>()
            .add_message::<ResetLevel>()
            .add_message::<NextLevel>()
            .add_message::<TrashCollected>()
            .add_message::<LoadLevel>();
    }
}
