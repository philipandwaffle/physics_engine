use bevy::{DefaultPlugins, prelude::App};

use crate::time_plugin::TimePlugin;

mod time_plugin;
mod components;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TimePlugin)
        .run();
}
