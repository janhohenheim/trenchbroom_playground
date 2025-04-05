use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(print_components);
}

#[derive(Event)]
pub struct PrintComponents;

fn print_components(trigger: Trigger<PrintComponents>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.run_system_cached_with(
        |In(entity): In<Entity>, world: &mut World| {
            info!(
                "{:#?}",
                world
                    .inspect_entity(entity)
                    .map(|info| info.name())
                    .collect::<Vec<_>>()
            );
        },
        entity,
    );
}
