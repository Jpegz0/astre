use bevy::prelude::*;

pub struct AstrePlugins;

impl Plugin for AstrePlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_world_system);
    }
}

fn hello_world_system() {
    println!("Hello world from Astre!");
}