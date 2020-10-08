use crate::helpers::stringify;

use dirs::data_local_dir;
use std::env;
use std::fs::create_dir;
use std::path::PathBuf;

fn get_env_pathbuf(var: &str) -> Option<PathBuf> {
    env::var_os(var).and_then(|value| {
        if value.is_empty() {
            None
        } else {
            Some(PathBuf::from(value))
        }
    })
}

pub fn project_dir() -> Result<PathBuf, String> {
    let dir_opt = {
        if let Some(devca_home) = get_env_pathbuf("DEVCA_HOME") {
            Some(devca_home)
        } else if let Some(mut data_dir) = data_local_dir() {
            data_dir.push("devca");
            Some(data_dir)
        } else {
            None
        }
    };
    match dir_opt {
        Some(dir) => {
            if !dir.exists() {
                create_dir(&dir).map_err(stringify)?;
            }
            Ok(dir)
        }
        None => {
            Err("Cannot find data directory. Please define DEVCA_HOME environment variable (i.e. `export DEVCA_HOME=$HOME/.devca`)".to_string())
        }
    }
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
