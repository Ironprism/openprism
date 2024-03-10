use anyhow::{Context, Result};
use rustdoc_types::Crate;
use std::path::Path;
use std::process::Command;
use regex::Regex;
use crate::utils::temp_dir;


/// Generate the documentation json for a crate in a temporary folder and
/// return the parsed `Crate` struct.
pub fn doc_crate<P: AsRef<Path>>(path: P) -> Result<Crate> {
    let tmp_dir = temp_dir().with_context(|| {
        format!(
            "Could not temp dir to document crate {}",
            path.as_ref().to_string_lossy()
        )
    })?;
    let res = doc_crate_with_dir(&path, &tmp_dir);
    std::fs::remove_dir_all(&tmp_dir)
        .map_err(|x| anyhow::anyhow!(x))
        .with_context(|| {
            format!(
                "Could not remmove temp dir {} which was used to document crate {}",
                tmp_dir.to_string_lossy(),
                path.as_ref().to_string_lossy()
            )
        })?;
    Ok(res?)
}

pub fn doc_crate_with_dir<P: AsRef<Path>, P2: AsRef<Path>>(path: P, tmp_dir: P2) -> Result<Crate> {
    log::debug!("Creating rustdoc json for {} @ dir {}", path.as_ref().to_string_lossy(), tmp_dir.as_ref().to_string_lossy());
    let output = Command::new("cargo")
        .args(&[
            "rustdoc",
            "-Z", 
            "unstable-options",
            "--output-format=json",
            &format!(
                "--manifest-path={}",
                path.as_ref().join("Cargo.toml").to_string_lossy()
            ),
            &format!("--target-dir={}", tmp_dir.as_ref().to_string_lossy()),
        ])
        .output()
        .with_context(|| {
            format!(
                "Could not run cargo rustdoc for crate {}",
                path.as_ref().to_string_lossy()
            )
        })?;
    log::debug!("Rustdoc exit code: {}", output.status.to_string());
    log::debug!("Rustdoc stdout: {}", String::from_utf8_lossy(&output.stdout));
    log::debug!("Rustdoc stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Cargo rustdoc failed for crate {}",
            path.as_ref().to_string_lossy()
        ));
    }

    let re = Regex::new(r"Generated (.+.json)").unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let cap = re.captures(&stderr)
        .with_context(|| format!("Could find the generated jsons in the rustdoc stderr"))?;
    log::debug!("Captures: {:#2?}", &cap);

    let json_path = &cap[1];
    log::debug!("Rustdoc json: {}", json_path);

    let json_file = std::fs::read_to_string(json_path)
        .with_context(|| format!("Could not read the generated rustdoc json"))?;
    log::trace!("Rustdoc json: {}", &json_file);
    std::fs::write("dbg.json", &json_file).unwrap();

    let krate = serde_json::from_str::<Crate>(&json_file).with_context(|| format!("Could not parse the generated rustdoc json"))?;
    log::trace!("Parsed rustdoc json: {:#2?}", krate);
    Ok(krate)
}
