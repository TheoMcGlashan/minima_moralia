use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Debug)]
struct GameCamera;

#[derive(Component, Debug)]
struct Viewpoint;

#[derive(Component, Debug)]
struct CameraPart;

#[derive(Resource, Debug)]
struct CursorPosition {
    lock_position: Vec2,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_viewpoint, set_lighting))
            .add_systems(Update, (camera_movement, cursor_lock))
            .insert_resource(CursorPosition { lock_position: Vec2::ZERO});
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, GameCamera, CameraPart));
}

fn spawn_viewpoint(mut commands: Commands) {
    commands.spawn((Viewpoint, CameraPart, Transform::from_xyz(-5.0, 0.0, -5.0)));
}

fn set_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 4000.;
}

fn cursor_lock(
    mut query: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut position: ResMut<CursorPosition>
) {
    let mut primary_window = query.single_mut();
    
    if mouse.just_pressed(MouseButton::Right) {
        if let Some(lock_position) = primary_window.cursor_position() {
            *position = CursorPosition { lock_position };
        } 

        primary_window.cursor.visible = false;
    } 
    else if mouse.pressed(MouseButton::Right) {
        primary_window.set_cursor_position(Some(position.lock_position));
    } else {
        primary_window.cursor.visible = true;
    }
}

fn camera_movement(mut query: Query<(&mut Transform, Has<GameCamera>), With<CameraPart>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut camera_position = Vec3::ZERO;
    let mut viewpoint_position = Vec3::ZERO;
    for (transform, is_camera) in query.iter() { 
        if is_camera {
            camera_position = transform.translation;
        } else {
            viewpoint_position = transform.translation;
        }
    }

    let look_vec = (viewpoint_position - camera_position).normalize();
    let cross_product = Vec3::new(-look_vec[2], 0.0, look_vec[0]).normalize();

    for (mut transform, _is_camera) in query.iter_mut() {

        // WASD and arrow key movement controls
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation -= cross_product * 10. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
           transform.translation += cross_product * 10. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation -= look_vec * 10. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation += look_vec * 10. * time.delta_seconds();
        }
    }
}