use bevy::prelude::*;

const fn get_main_camera_fov_degrees() -> f32 {
    60.0
}
fn get_main_camera_fov_radians() -> f32 {
    get_main_camera_fov_degrees().to_radians()
}

#[derive(Component)]
struct MarkerLightMain;

#[derive(Component)]
struct MarkerMeshCube1;

#[derive(Component)]
struct MarkerMeshCircle1;

#[derive(Component)]
struct MarkerCameraMain;

fn system_setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Point Light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        MarkerLightMain,
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_length(1.0)),
            material: materials.add(Color::srgb_u8(0xe9, 0x21, 0x7a)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        MarkerMeshCube1,
    ));

    // Circular base
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(4.0, 0.1)),
            material: materials.add(Color::WHITE),
            // transform: Transform::from_rotation(Quat::from_rotation_x(
            //     -std::f32::consts::FRAC_PI_2,
            // )),
            ..default()
        },
        MarkerMeshCircle1,
    ));

    // Camera
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: get_main_camera_fov_radians(),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(-4.0, 3.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MarkerCameraMain,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, system_setup_3d_scene)
        .run();
}
