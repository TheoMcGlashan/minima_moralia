use bevy::prelude::*;

pub struct AphorismsPlugin;

impl Plugin for AphorismsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // square
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.1, 1.0, 1.0)),
        material: materials.add(Color::WHITE),
        ..default()
    });
}
