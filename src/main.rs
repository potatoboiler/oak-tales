use bevy::prelude::*;

mod state;
use state::StateHandler;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "MapleS2ory".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(StateHandler)
        .run();
}
