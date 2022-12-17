use bevy::prelude::*;
// use bevy_flycam::PlayerPlugin;
use day_14::render::VisualizationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(PlayerPlugin)
        .add_plugin(VisualizationPlugin)
        .run();
}
