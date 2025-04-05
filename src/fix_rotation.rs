use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::PointClass;

pub fn plugin(app: &mut App) {
    app.register_type::<FixTrenchBroomRotation>();
}

#[derive(PointClass, Component, Default, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component, Default, PartialEq, Debug)]
#[component(on_add = Self::on_add)]
pub struct FixTrenchBroomRotation;

impl FixTrenchBroomRotation {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let rotation = Quat::from_rotation_y(std::f32::consts::TAU / 8.);
        if let Some(mut transform) = world.get_mut::<Transform>(entity) {
            transform.rotate(rotation);
        } else {
            world.commands().entity(entity).insert(Transform {
                rotation,
                ..default()
            });
        }
    }
}
