use bevy::prelude::*;
pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.9, 0.0)));
    }
}
