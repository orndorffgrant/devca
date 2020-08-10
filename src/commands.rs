use dirs::data_local_dir;
use std::path::PathBuf;
use std::fs::create_dir;

fn project_dir() -> Result<PathBuf, &'static str> {
    let mut dir = data_local_dir().ok_or("Cannot find data directory")?;
    dir.push("devca");
    if !dir.exists() {
        create_dir(&dir).map_err(|_| { "failed to create projects dir" })?;
    }
    Ok(dir)
}

fn ca_dir() -> Result<PathBuf, &'static str> {
    let mut dir = project_dir()?;
    dir.push("ca");
    if !dir.exists() {
        create_dir(&dir).map_err(|_| { "failed to create ca dir" })?;
    }
    Ok(dir)
}

fn certs_dir() -> Result<PathBuf, &'static str> {
    let mut dir = project_dir()?;
    dir.push("certs");
    if !dir.exists() {
        create_dir(&dir).map_err(|_| { "failed to create certs dir" })?;
    }
    Ok(dir)
}

fn cert_dir(name: &str) -> Result<PathBuf, &'static str> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if !dir.exists() {
        create_dir(&dir).map_err(|_| { "failed to create cert dir" })?;
    }
    Ok(dir)
}

// fn get_ca() -> Result<PathBuf, &'static str> {
// }

pub(crate) fn new_cert(name: &str) -> Result<(), &'static str>{
    let cert_dir = cert_dir(name)?;

    // TODO check if CA exists
    // TODO create CA if doesn't exist
    // TODO check if cert name exists
    // TODO prompt if cert name already exists
    // TODO create cert
    // TODO print results
    println!("{}", cert_dir.to_str().unwrap());
    Ok(())
}