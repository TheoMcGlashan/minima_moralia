use bevy::prelude::*;
use camera::CameraPlugin;
use aphorisms::AphorismsPlugin;
mod camera;
mod aphorisms;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AphorismsPlugin)
        .run();
}