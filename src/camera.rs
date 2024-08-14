use bevy::{input::mouse::MouseMotion, prelude::*, ui::update};

#[derive(Component, Debug)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, set_lighting))
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, GameCamera));
}

fn set_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 4000.;
}

fn camera_movement(mut query: Query<&mut Transform, With<GameCamera>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mouse_movement_event: EventReader<MouseMotion>,
    time: Res<Time>
) {
    for mut transform in query.iter_mut() {

        // WASD and arrow key movement controls
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation[0] += 10. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
           transform.translation[0] -= 10. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            // translate backwards
        }
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            // translate forwards
        }

        // mouse controls to look around
        if mouse_button_input.pressed(MouseButton::Right) {
            // deal with rotation stuff
        }
    }
}