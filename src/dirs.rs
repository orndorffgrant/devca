use crate::helpers::stringify;
use dirs::data_local_dir;

use std::fs::{create_dir};
use std::path::PathBuf;

pub fn project_dir() -> Result<PathBuf, String> {
    let mut dir = data_local_dir().ok_or("Cannot find data directory")?;
    dir.push("devca");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

pub fn ca_dir() -> Result<PathBuf, String> {
    let mut dir = project_dir()?;
    dir.push("ca");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

pub fn certs_dir() -> Result<PathBuf, String> {
    let mut dir = project_dir()?;
    dir.push("certs");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

pub fn cert_dir(name: &str) -> Result<PathBuf, String> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}