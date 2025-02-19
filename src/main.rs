//! Avalanche Living
//! Loosely based on beloved advertisement, ******** ********
//! Fun Fact: I don't know what I'm doing. At all.

use bevy::prelude::*;

use game::GamePlugin;

mod game;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins,
                GamePlugin,
        ))
        .run();
}
