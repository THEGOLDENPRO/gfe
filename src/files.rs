use std::{path::Path, sync::Arc};

use rfd::AsyncFileDialog;
use tokio::fs;

use crate::GFEError;

pub async fn pick_file() -> Result<Arc<String>, GFEError> {
    let path = AsyncFileDialog::new().set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(GFEError::DialogClosed)?;

    load_file(path.path()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, GFEError> {
    fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(GFEError::IO)
}