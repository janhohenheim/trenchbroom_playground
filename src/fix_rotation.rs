use bevy::{
    prelude::*,
    scene::{SceneInstance, SceneInstanceReady},
};
use bevy_trenchbroom::prelude::PointClass;

pub fn plugin(app: &mut App) {
    app.register_type::<FixTrenchBroomRotation>();
    app.add_observer(fix_rotation);
}

#[derive(PointClass, Component, Default, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component, Default, PartialEq, Debug)]
pub struct FixTrenchBroomRotation;

fn fix_rotation(
    trigger: Trigger<SceneInstanceReady>,
    q_rotation_fix: Query<Entity, With<FixTrenchBroomRotation>>,
    q_children: Query<&Children>,
    mut q_transform: Query<&mut Transform>,
) {
    let entity = trigger.entity();
    if !q_rotation_fix.contains(entity) {
        return;
    }
    let children = q_children.get(entity).unwrap();
    let rotation = Quat::from_rotation_y(std::f32::consts::TAU / 4.);
    for child in children {
        if let Ok(mut transform) = q_transform.get_mut(*child) {
            transform.rotate(rotation);
        }
    }
}
