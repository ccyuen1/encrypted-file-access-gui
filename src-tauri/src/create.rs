use std::path::PathBuf;

use encrypted_file_access::create::CreateArgs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// Copy of CreateArgs to allow deriving [`Deserialize`]
pub struct CreateArgsDef {
    out_file: PathBuf,
    src: Option<PathBuf>,
    extension: Option<String>,
    no_compress: bool,
    xz_level: u32,
}

impl From<CreateArgsDef> for CreateArgs {
    fn from(value: CreateArgsDef) -> Self {
        CreateArgs {
            out_file: value.out_file,
            src: value.src,
            extension: value.extension,
            no_compress: value.no_compress,
            xz_level: value.xz_level,
        }
    }
}

#[tauri::command]
pub fn create(args: CreateArgsDef, password: String) -> Result<(), String> {
    encrypted_file_access::create::create(&args.into(), Some(password.into()))
        .map_err(|e| e.to_string())
}
