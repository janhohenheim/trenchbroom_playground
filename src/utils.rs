use bevy::prelude::*;
use bevy_trenchbroom::class::QuakeClass;

pub fn plugin(_app: &mut App) {}

pub trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        let model = T::CLASS_INFO.model.unwrap().trim_matches('"');
        self.load(format!("{model}#Scene0"))
    }
}
