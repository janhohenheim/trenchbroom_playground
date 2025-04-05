use std::path::PathBuf;

use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, write_trenchbroom_config);
}

fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    {
        let dir = trenchbroom_path("trenchbroom_playground").unwrap();
        info!("Writing TrenchBroom config to {}", dir.display());
        if let Err(err) = server.config.write_folder(dir) {
            error!("Could not write TrenchBroom config: {err}");
        }
    }
}

fn trenchbroom_path(game_name: &str) -> Result<PathBuf, TrenchBroomPathError> {
    // Source: https://trenchbroom.github.io/manual/latest/#game_configuration_files
    let trenchbroom_userdata = if cfg!(target_os = "linux") {
        #[allow(deprecated)] // No longer deprecated starting from 1.86
        std::env::home_dir().map(|path| path.join(".TrenchBroom"))
    } else if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .ok()
            .map(|path| PathBuf::from(path).join("TrenchBroom"))
    } else if cfg!(target_os = "macos") {
        #[allow(deprecated)] // No longer deprecated starting from 1.86
        std::env::home_dir().map(|path| {
            path.join("Library")
                .join("Application Support")
                .join("TrenchBroom")
        })
    } else {
        return Err(TrenchBroomPathError::UnsupportedOs(
            std::env::consts::OS.to_string(),
        ));
    };

    let Some(trenchbroom_userdata) = trenchbroom_userdata else {
        return Err(TrenchBroomPathError::HomeDirNotFound);
    };

    if !trenchbroom_userdata.exists() {
        return Err(TrenchBroomPathError::UserDataNotFound(trenchbroom_userdata));
    }

    let trenchbroom_game_config = trenchbroom_userdata.join("games").join(game_name);

    if !trenchbroom_game_config.exists() {
        let err = std::fs::create_dir_all(&trenchbroom_game_config);
        if let Err(err) = err {
            return Err(TrenchBroomPathError::CreateDirError(err));
        }
    }

    Ok(trenchbroom_game_config)
}

#[derive(Debug)]
enum TrenchBroomPathError {
    UnsupportedOs(String),
    HomeDirNotFound,
    UserDataNotFound(PathBuf),
    CreateDirError(std::io::Error),
}

impl std::fmt::Display for TrenchBroomPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedOs(os) => write!(f, "Unsupported target OS: {os}"),
            Self::HomeDirNotFound => write!(f, "Home directory not found"),
            Self::UserDataNotFound(path) => {
                write!(
                    f,
                    "TrenchBroom user data not found at {}. Have you installed TrenchBroom?",
                    path.display()
                )
            }
            Self::CreateDirError(err) => write!(f, "Failed to create game config directory: {err}"),
        }
    }
}

impl std::error::Error for TrenchBroomPathError {}
