use crate::certs::{create_ca, create_cert};
use crate::dirs::{ca_dir, cert_dir, certs_dir};
use crate::helpers::stringify;

use serde::{Deserialize, Serialize};
use std::fs::{read, write};
use std::io;
use std::io::prelude::*;

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

#[derive(Serialize, Deserialize)]
struct CAState {
    serial_number: u32,
}

fn get_ca_state() -> Result<CAState, String> {
    let mut ca_state_path = ca_dir()?;
    ca_state_path.push("state.json");
    let mut curr_ca_state = {
        if ca_state_path.exists() {
            let raw_ca_state = read(&ca_state_path).map_err(stringify)?;
            serde_json::from_slice(&raw_ca_state).map_err(stringify)?
        } else {
            CAState { serial_number: 0 }
        }
    };

    curr_ca_state.serial_number += 1;

    let serialized_ca_state = serde_json::to_vec(&curr_ca_state).map_err(stringify)?;

    write(&ca_state_path, serialized_ca_state).map_err(stringify)?;

    Ok(curr_ca_state)
}

pub(crate) fn new_cert(name: &str) -> Result<(), String> {
    let ca_key_pem = get_ca_key()?;
    let ca_state = get_ca_state()?;

    let mut key_path = cert_dir(name)?;
    let mut cert_path = key_path.clone();
    key_path.push("key.pem");
    cert_path.push("cert.pem");

    if key_path.exists() {
        print!("**** A certificate for \"localhost\" already exists. Would you like to overwrite it? y/N: ");
        io::stdout().flush().map_err(stringify)?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).map_err(stringify)?;
        if answer.to_ascii_lowercase().chars().nth(0) != Some('y') {
            return Ok(());
        }
    }

    let (cert_pem, key_pem) = create_cert(name, &ca_key_pem, ca_state.serial_number)?;
    write(&key_path, &key_pem).map_err(stringify)?;
    write(&cert_path, &cert_pem).map_err(stringify)?;

    println!(
        "Created private key for \"{}\": {}",
        name,
        key_path.to_str().unwrap()
    );
    println!(
        "Created certificate for \"{}\": {}",
        name,
        cert_path.to_str().unwrap()
    );
    Ok(())
}

pub(crate) fn ls() -> Result<(), String> {
    let dir = certs_dir()?;
    let dir_iterator = dir.read_dir().map_err(stringify)?;
    for cert_dir in dir_iterator {
        let cert_dir = cert_dir.map_err(stringify)?;
        let cert_dir_path = cert_dir.path();

        let mut cert_dir_key_path = cert_dir_path.clone();
        let mut cert_dir_cert_path = cert_dir_path.clone();
        cert_dir_key_path.push("key.pem");
        cert_dir_cert_path.push("cert.pem");

        if cert_dir_key_path.exists() && cert_dir_cert_path.exists() {
            let cert_name_os = cert_dir_path
                .file_name()
                .ok_or("error getting directory name")?;
            let cert_name = cert_name_os
                .to_str()
                .ok_or("error converting directory name")?;

            println!("{}", cert_name);
        }
    }
    Ok(())
}

pub(crate) fn path_to(name: &str) -> Result<(), String> {
    let mut dir = certs_dir()?;
    dir.push(name);
    if dir.exists() {
        println!("{}", dir.to_str().ok_or("error converting directory name")?);
        Ok(())
    } else {
        Err(format!(
            "Cert with that name has not been created. Create it with \"devca new {}\"",
            name
        ))
    }
}
