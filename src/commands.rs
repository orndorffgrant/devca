use crate::helpers::stringify;
use crate::certs::{create_ca, create_cert};
use dirs::data_local_dir;
use std::fs::{read, write, create_dir};
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

fn get_ca_key() -> Result<Vec<u8>, String> {
    let mut key_path = ca_dir()?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");
    let key_pem = {
        if key_path.exists() {
            read(key_path).map_err(stringify)?
        } else {
            let (ca_cert_pem, ca_key_pem) = create_ca()?;
            write(&key_path, &ca_key_pem).map_err(stringify)?;
            write(&cert_path, &ca_cert_pem).map_err(stringify)?;
            println!("Created CA private key: {}", key_path.to_str().unwrap());
            println!("Created CA certificate: {}", cert_path.to_str().unwrap());
            ca_key_pem
        }
    };
    Ok(key_pem)
}

pub(crate) fn new_cert(name: &str) -> Result<(), String> {
    let ca_key_pem = get_ca_key()?;
    let mut key_path = cert_dir(name)?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");
    // TODO check if cert name exists
    // TODO prompt if cert name already exists
    let (cert_pem, key_pem) = create_cert(name, &ca_key_pem)?;
    write(&key_path, &key_pem).map_err(stringify)?;
    write(&cert_path, &cert_pem).map_err(stringify)?;

    println!("Created private key for \"{}\": {}", name, key_path.to_str().unwrap());
    println!("Created certificate for \"{}\": {}", name, cert_path.to_str().unwrap());
    Ok(())
}
