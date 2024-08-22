
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

const VIEWPOINT_DISTANCE: f32 = 10.0;

// Marker struct for the game camera.
#[derive(Component, Debug)]
struct GameCamera;

// Marker struct for the viewpoint, the point the camera looks at and rotates around.
#[derive(Component, Debug)]
struct Viewpoint;

// Marker used when querying for the camera and viewpoint.
#[derive(Component, Debug)]
struct CameraPart;

// Resource that stores the cursor position to lock the cursor in place.
#[derive(Resource, Debug)]
struct MouseLock {
    lock_position: Vec2,
    lock_bool: bool,
}

// Resource to keep track of 
#[derive(Resource, Debug)]
struct UpVec {
    up: Vec3,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_viewpoint, set_lighting))
            .add_systems(Update, (camera_movement, cursor_lock))
            .insert_resource(MouseLock { lock_position: Vec2::ZERO, lock_bool: false})
            .insert_resource(UpVec { up: Vec3::new(0.0, 1.0, 0.0)});
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
    mut mouse_lock: ResMut<MouseLock>
) {
    let mut primary_window = query.single_mut();
    
    if mouse.just_pressed(MouseButton::Right) {
        if let Some(lock_position) = primary_window.cursor_position() {
            *mouse_lock = MouseLock { lock_position, lock_bool: true};
        } 

        primary_window.cursor.visible = false;
    } 
    else if mouse.pressed(MouseButton::Right) {
        primary_window.set_cursor_position(Some(mouse_lock.lock_position));
    } else {
        primary_window.cursor.visible = true;
        *mouse_lock = MouseLock { lock_position: mouse_lock.lock_position, lock_bool: false };
    }
}

fn camera_movement(mut query: Query<(&mut Transform, Has<GameCamera>), With<CameraPart>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    mut up_vec_res: ResMut<UpVec>,
    mouse_lock: ResMut<MouseLock>,
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
    let cross_product = Vec3::new(look_vec[1] * up_vec_res.up[2] - look_vec[2] * up_vec_res.up[1],
                                        look_vec[2] * up_vec_res.up[0] - look_vec[0] * up_vec_res.up[2],
                                        look_vec[0] * up_vec_res.up[1] - look_vec[1] * up_vec_res.up[0]).normalize();

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

        // Mouse rotational movement controls

        if mouse_lock.lock_bool {
            let mut mouse_vec = Vec2::ZERO;
            for ev in evr_mouse_motion.read() {
                mouse_vec = Vec2::new(ev.delta.x, ev.delta.y);
            }

            let camera_relative_position = camera_position - viewpoint_position;
            let move_vec = (up_vec_res.up * mouse_vec[1] + cross_product * mouse_vec[0]);
            println!("{}", move_vec);
            let rotation_arc = (mouse_vec[0].powi(2) + mouse_vec[1].powi(2)).sqrt() / VIEWPOINT_DISTANCE;
            let new_camera_position = Vec3::new(camera_relative_position[0] * rotation_arc.cos() + VIEWPOINT_DISTANCE * move_vec[0] * rotation_arc.sin(),
                                                      camera_relative_position[1] * rotation_arc.cos() + VIEWPOINT_DISTANCE * move_vec[1] * rotation_arc.sin(),
                                                      camera_relative_position[2] * rotation_arc.cos() + VIEWPOINT_DISTANCE * move_vec[2] * rotation_arc.sin())
                                                    + viewpoint_position;    
            let new_look_vec = viewpoint_position - new_camera_position;
            let new_cross_product = Vec3::new(-new_look_vec[2], 0.0, new_look_vec[0]);
            let new_up = Vec3::new(new_look_vec[2] * new_cross_product[1] - new_look_vec[1] * new_cross_product[2],
                                         new_look_vec[0] * new_cross_product[2] - new_look_vec[2] * new_cross_product[0],
                                         new_look_vec[1] * new_cross_product[0] - new_look_vec[0] * new_cross_product[1]).normalize();
    
            *up_vec_res = UpVec { up: new_up };

            let camera_movement = new_camera_position - camera_position;
    
            transform.translation += camera_movement;
        }
    }
}