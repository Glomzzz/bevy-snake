use bevy::prelude::*;

use game::GamePlugin;

mod game;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
