use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

#[cfg(debug_assertions)]
mod dev;
#[cfg(debug_assertions)]
use dev::PrintComponents;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(TrenchBroomPlugin(TrenchBroomConfig::new(
            "trenchbroom_playground",
        )))
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, spawn_map);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(dev::plugin);
        app.add_systems(
            Update,
            print_suzanne_components.run_if(input_just_pressed(KeyCode::Space)),
        );
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

#[derive(PointClass, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/Suzanne.gltf")]
#[component(on_add = Self::on_add)]
pub struct Suzanne;

impl Suzanne {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };

        let model = Suzanne::CLASS_INFO.model.unwrap().trim_matches('"');
        let suzanne = asset_server.load(format!("{model}#Scene0"));

        world.commands().entity(entity).insert((
            SceneRoot(suzanne),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
            TrenchBroomGltfRotationFix,
        ));
    }
}

fn print_suzanne_components(mut commands: Commands, q_suzanne: Query<Entity, With<Suzanne>>) {
    for entity in q_suzanne.iter() {
        commands.trigger_targets(PrintComponents, entity);
    }
}
