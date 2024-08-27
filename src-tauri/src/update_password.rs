use std::path::PathBuf;

use encrypted_file_access::change_password::ChangePasswordArgs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// Copy of [`ChangePasswordArgs`] to allow deriving [`Deserialize`]
pub struct ChangePasswordArgsDef {
    file: PathBuf,
    no_compress: Option<bool>,
    xz_level: Option<u32>,
}

impl From<ChangePasswordArgsDef> for ChangePasswordArgs {
    fn from(value: ChangePasswordArgsDef) -> Self {
        ChangePasswordArgs {
            file: value.file,
            no_compress: value.no_compress,
            xz_level: value.xz_level,
        }
    }
}

#[tauri::command]
pub async fn update_password(
    args: ChangePasswordArgsDef,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        encrypted_file_access::change_password::change_password(
            &args.into(),
            Some(old_password.into()),
            Some(new_password.into()),
        )
        .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
