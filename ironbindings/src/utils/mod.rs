use anyhow::{Context, Result};

mod counter;
pub use counter::*;

mod twine;
pub use twine::*;

mod unwrap_as;
pub use unwrap_as::*;

/// read std::env::dir with an unique name
pub fn temp_dir() -> Result<std::path::PathBuf> {
    let mut path = std::env::temp_dir();
    path.push(std::path::Path::new(&format!(
        "irontraits-{}",
        uuid::Uuid::new_v4()
    )));
    log::debug!("Creating temp dir {}", path.to_string_lossy());
    std::fs::create_dir(&path)
        .with_context(|| format!("Could not create temp dir {}", path.to_string_lossy()))?;
    Ok(path)
}
