use std::path::PathBuf;

use encrypted_file_access::open::OpenArgs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// Copy of [`OpenArgs`] to allow deriving [`Deserialize`]
pub struct OpenArgsDef {
    file: PathBuf,
    executable: Option<PathBuf>,
}

impl From<OpenArgsDef> for OpenArgs {
    fn from(value: OpenArgsDef) -> Self {
        OpenArgs {
            file: value.file,
            executable: value.executable,
        }
    }
}

#[tauri::command]
pub async fn open(args: OpenArgsDef, password: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        encrypted_file_access::open::open(&args.into(), Some(password.into()))
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
