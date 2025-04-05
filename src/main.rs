use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrenchBroomPlugin(TrenchBroomConfig::new(
            "trenchbroom_playground",
        )))
        .add_systems(Startup, spawn_map)
        .run();
}
fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneRoot(asset_server.load("maps/playground.map#Scene")));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
    commands.spawn((DirectionalLight {
        shadows_enabled: true,
        ..default()
    },));
}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle())]
pub struct Worldspawn;
