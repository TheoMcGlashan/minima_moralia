
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraPlugin;

// Plugin to add camera systems to our app.
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, set_lighting))
            .add_systems(Update, pan_orbit_camera);
    }
}

// Component to represent our 3d camera, which is a default Camera3dBundle
// with state and settings added to allow for camera panning and orbiting.
#[derive(Bundle, Default)]
struct PanOrbitCameraBundle {
    camera: Camera3dBundle,
    state: PanOrbitState,
    settings: PanOrbitSettings,
}

// Component that holds variables useed to calculate the camera's position
// and viewpoint when orbiting and panning.
#[derive(Component)]
struct PanOrbitState {
    center: Vec3,
    pitch: f32,
    yaw: f32,
    radius: f32,
    upside_down: bool,
}

// Component that holds variables used to calculate the speed of the
// camera's panning and orbiting.
#[derive(Component)]
struct PanOrbitSettings {
    pan_sensitivity: f32,
    orbit_sensitivity: f32,
}

// Default values for the camera state, these will in general not
// be changed while the app is running.
impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::new(1.0, 1.0, 1.0),
            pitch: 15.0f32.to_radians(),
            yaw: 30.0f32.to_radians(),
            radius: 5.0,
            upside_down: false,
        }
    }
}

// Default values for the camera settings, these can be changed
// while the app is running.
impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.1,
            orbit_sensitivity: 0.2f32.to_radians(),
        }
    }
}

// Spawn our camera with default values.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(PanOrbitCameraBundle::default());
}

// Create ambient lighting that evenly lights the world.
fn set_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 4000.;
}

// Function to handle inputs to pan and orbit the camera and calculate
// the camera's new position and viewpoint after moving.
fn pan_orbit_camera(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut q_camera: Query<(
        &PanOrbitSettings,
        &mut PanOrbitState,
        &mut Transform,
    )>
) {

    // Store the x and y values for cursor movement to use in orbiting.
    let total_motion: Vec2 = evr_motion.read()
        .map(|ev| ev.delta).sum();

    for (settings, mut state, mut transform) in &mut q_camera {

        // If right click is pressed, store the product of the mouse motion
        // and the orbit sensitivity in a variable to use for rot.
        // Also, check if the camera is upside down and set a bool.
        let mut total_orbit = Vec2::ZERO;
        if mouse_button.pressed(MouseButton::Right) {
            total_orbit -= total_motion * settings.orbit_sensitivity;
            state.upside_down = state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2;
        }

        // If the camera is upside down, orbit the camera the opposite direction.
        // This effectively prevents the camera from being upside down.
        if state.upside_down {
            total_orbit.x = -total_orbit.x;
        }

        // Bool to check if any camera movement is needed.
        let mut any = false;

        // if the camera is to be orbited, modify it's pitch and yaw to rotate it.
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw += total_orbit.x;
            state.pitch += total_orbit.y;

            // This prevents the camera's pitch and yaw from accumulating as the
            // camera orbits, as we only care about pitch and yaw mod 
            if state.yaw > PI {
                state.yaw -= TAU;
            }
            if state.yaw < -PI {
                state.yaw += TAU;
            }
            if state.pitch > PI {
                state.pitch -= TAU;
            }
            if state.pitch < -PI {
                state.pitch += TAU;
            }
        }

        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            any = true;
            state.center += transform.left() * settings.pan_sensitivity;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            any = true;
            state.center += transform.right() * settings.pan_sensitivity;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            any = true;
            state.center += transform.back() * settings.pan_sensitivity;
        }
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            any = true;
            state.center += transform.forward() * settings.pan_sensitivity;
        }
        if keyboard_input.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            any = true;
            state.center += transform.up() * settings.pan_sensitivity;
        }
        if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            any = true;
            state.center += transform.down() * settings.pan_sensitivity;
        }

        if any || state.is_added() {
            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}