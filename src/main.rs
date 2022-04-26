use bevy::prelude::*;
use bevy::window::PresentMode;

mod state;
use state::StateHandler;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "MapleS2ory".to_string(),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(StateHandler)
        .run();
}
