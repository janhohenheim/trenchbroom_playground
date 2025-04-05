use bevy::prelude::*;

mod print_components;

pub use self::print_components::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((print_components::plugin,));
}
