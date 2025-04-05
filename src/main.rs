use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;
use fix_rotation::FixTrenchBroomRotation;

mod fix_rotation;
#[cfg(debug_assertions)]
mod trenchbroom_config;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(TrenchBroomPlugin(TrenchBroomConfig::new(
            "trenchbroom_playground",
        )))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(fix_rotation::plugin)
        .add_systems(Startup, spawn_map);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(trenchbroom_config::plugin);
    }

    app.run();
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("maps/playground.map#Scene")));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(1.0, -10.0, 0.0), Vec3::Y),
    ));
}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle())]
pub struct Worldspawn;

const SUZANNE_MODEL: &str = "models/Suzanne.gltf";

#[derive(PointClass, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility, FixTrenchBroomRotation)]
#[model("models/Suzanne.gltf")]
#[component(on_add = Self::on_add)]
pub struct Suzanne;

impl Suzanne {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };

        let suzanne = asset_server.load(format!("{SUZANNE_MODEL}#Scene0"));

        world.commands().entity(entity).insert((
            SceneRoot(suzanne),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
        ));
    }
}
