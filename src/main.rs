use bevy::prelude::{App, DefaultPlugins};

mod resources;
use resources::window::window_descriptor_builder;

fn main() {
    App::new()
        .insert_resource(window_descriptor_builder())
        .add_plugins(DefaultPlugins)
        .run();
}
