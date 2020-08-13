use crate::helpers::stringify;
use crate::certs::{create_ca, create_cert};
use dirs::data_local_dir;
use std::fs::create_dir;
use std::path::PathBuf;

fn project_dir() -> Result<PathBuf, String> {
    let mut dir = data_local_dir().ok_or("Cannot find data directory")?;
    dir.push("devca");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

fn ca_dir() -> Result<PathBuf, String> {
    let mut dir = project_dir()?;
    dir.push("ca");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

fn certs_dir() -> Result<PathBuf, String> {
    let mut dir = project_dir()?;
    dir.push("certs");
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

fn cert_dir(name: &str) -> Result<PathBuf, String> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if !dir.exists() {
        create_dir(&dir).map_err(stringify)?;
    }
    Ok(dir)
}

// fn get_ca() -> Result<PathBuf, &'static str> {
// }

pub(crate) fn new_cert(name: &str) -> Result<(), String> {
    let cert_dir = cert_dir(name)?;
    // TODO check if CA exists
    // TODO create CA if doesn't exist
    let (ca_cert_pem, ca_key_pem) = create_ca()?;
    let (cert_pem, cert_key_pem) = create_cert(name, &ca_key_pem)?;

    // TODO check if cert name exists
    // TODO prompt if cert name already exists
    // TODO create cert
    // TODO print results
    println!("{}", cert_dir.to_str().unwrap());
    println!("{}", String::from_utf8(ca_cert_pem).map_err(stringify)?);
    println!("{}", String::from_utf8(cert_pem).map_err(stringify)?);
    Ok(())
}
