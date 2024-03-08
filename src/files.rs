use std::{path::PathBuf, sync::Arc};

use tokio::fs;
use rfd::AsyncFileDialog;

use crate::GFEError;

pub async fn pick_file() -> Result<(PathBuf, Arc<String>), GFEError> {
    let file_handle = AsyncFileDialog::new().set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(GFEError::DialogClosed)?;

    load_file(file_handle.path().to_owned()).await
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), GFEError> {
    let contents = fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(GFEError::IO)?;

    Ok((path, contents))
}